use shared::{BatchOperation, Operation, StateRoot, ExecutionResult, DeFiResult, DeFiHubError, TokenType};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use std::collections::VecDeque;
use tracing::{info, debug, warn};

/// 30초마다 실행되는 배치 프로세서
pub struct BatchProcessor {
    /// 대기 중인 작업들
    pending_operations: VecDeque<Operation>,
    
    /// 처리된 배치들 (최근 100개)
    processed_batches: VecDeque<BatchOperation>,
    
    /// 현재 상태 루트
    current_state_root: StateRoot,
    
    /// 다음 배치 처리 시간
    next_batch_time: DateTime<Utc>,
    
    /// 최대 배치 크기
    max_batch_size: usize,
}

impl BatchProcessor {
    /// 새로운 배치 프로세서 생성
    pub fn new() -> Self {
        Self {
            pending_operations: VecDeque::new(),
            processed_batches: VecDeque::new(),
            current_state_root: StateRoot {
                hash: [0; 32],
                height: 0,
                timestamp: Utc::now(),
            },
            next_batch_time: Utc::now() + Duration::seconds(30),
            max_batch_size: 1000,
        }
    }
    
    /// 작업 추가
    pub fn add_operation(&mut self, operation: Operation) {
        debug!("Adding operation to batch queue: {:?}", operation);
        self.pending_operations.push_back(operation);
    }
    
    /// 배치 처리 시간인지 확인
    pub fn should_process_batch(&self) -> bool {
        let now = Utc::now();
        now >= self.next_batch_time || self.pending_operations.len() >= self.max_batch_size
    }
    
    /// 배치 처리 실행
    pub fn process_batch(&mut self) -> DeFiResult<BatchOperation> {
        if self.pending_operations.is_empty() {
            return Err(DeFiHubError::BatchProcessing("No operations to process".to_string()));
        }
        
        info!("Processing batch with {} operations", self.pending_operations.len());
        
        // 현재 배치에 포함될 작업들 수집
        let mut operations = Vec::new();
        let batch_size = std::cmp::min(self.pending_operations.len(), self.max_batch_size);
        
        for _ in 0..batch_size {
            if let Some(op) = self.pending_operations.pop_front() {
                operations.push(op);
            }
        }
        
        // 새로운 상태 루트 계산
        let new_state_root = self.calculate_new_state_root(&operations)?;
        
        // 배치 생성
        let batch = BatchOperation {
            id: Uuid::new_v4(),
            operations,
            timestamp: Utc::now(),
            previous_state_root: self.current_state_root.clone(),
            new_state_root: new_state_root.clone(),
            signature: None, // TODO: 서명 추가
        };
        
        // 상태 업데이트
        self.current_state_root = new_state_root;
        self.next_batch_time = Utc::now() + Duration::seconds(30);
        
        // 처리된 배치 저장 (최근 100개만 유지)
        self.processed_batches.push_back(batch.clone());
        if self.processed_batches.len() > 100 {
            self.processed_batches.pop_front();
        }
        
        info!("Batch processed successfully: {}", batch.id);
        Ok(batch)
    }
    
    /// 새로운 상태 루트 계산
    fn calculate_new_state_root(&self, operations: &[Operation]) -> DeFiResult<StateRoot> {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        
        // 이전 상태 루트 해시
        hasher.update(&self.current_state_root.hash);
        
        // 모든 작업들의 해시
        for operation in operations {
            let op_bytes = serde_json::to_vec(operation)
                .map_err(|e| DeFiHubError::Serialization(e.to_string()))?;
            hasher.update(&op_bytes);
        }
        
        // 타임스탬프 추가
        let timestamp = Utc::now();
        hasher.update(&timestamp.timestamp().to_le_bytes());
        
        let hash: [u8; 32] = hasher.finalize().into();
        
        Ok(StateRoot {
            hash,
            height: self.current_state_root.height + 1,
            timestamp,
        })
    }
    
    /// 현재 상태 조회
    pub fn get_current_state(&self) -> &StateRoot {
        &self.current_state_root
    }
    
    /// 대기 중인 작업 수
    pub fn pending_operations_count(&self) -> usize {
        self.pending_operations.len()
    }
    
    /// 다음 배치 처리까지 남은 시간 (초)
    pub fn time_until_next_batch(&self) -> i64 {
        let now = Utc::now();
        if now >= self.next_batch_time {
            0
        } else {
            (self.next_batch_time - now).num_seconds()
        }
    }
    
    /// 최근 처리된 배치들 조회
    pub fn get_recent_batches(&self, limit: usize) -> Vec<&BatchOperation> {
        self.processed_batches
            .iter()
            .rev()
            .take(limit)
            .collect()
    }
    
    /// 특정 배치 조회
    pub fn get_batch(&self, batch_id: &Uuid) -> Option<&BatchOperation> {
        self.processed_batches
            .iter()
            .find(|batch| &batch.id == batch_id)
    }
    
    /// 배치 처리 통계
    pub fn get_statistics(&self) -> BatchStatistics {
        let total_batches = self.processed_batches.len();
        let total_operations: usize = self.processed_batches
            .iter()
            .map(|batch| batch.operations.len())
            .sum();
        
        let avg_operations_per_batch = if total_batches > 0 {
            total_operations as f64 / total_batches as f64
        } else {
            0.0
        };
        
        BatchStatistics {
            total_batches,
            total_operations,
            pending_operations: self.pending_operations.len(),
            avg_operations_per_batch,
            current_height: self.current_state_root.height,
            next_batch_in_seconds: self.time_until_next_batch(),
        }
    }
}

/// 배치 처리 통계
#[derive(Debug, Clone)]
pub struct BatchStatistics {
    pub total_batches: usize,
    pub total_operations: usize,
    pub pending_operations: usize,
    pub avg_operations_per_batch: f64,
    pub current_height: u64,
    pub next_batch_in_seconds: i64,
}

/// 작업 검증기
pub struct OperationValidator;

impl OperationValidator {
    /// 작업이 유효한지 검증
    pub fn validate_operation(operation: &Operation) -> DeFiResult<()> {
        match operation {
            Operation::Deposit { amount, .. } => {
                if amount.to_sat() == 0 {
                    return Err(DeFiHubError::Internal("Deposit amount cannot be zero".to_string()));
                }
            },
            Operation::Withdraw { amount, .. } => {
                if amount.to_sat() == 0 {
                    return Err(DeFiHubError::Internal("Withdrawal amount cannot be zero".to_string()));
                }
            },
            Operation::Swap { amount_in, min_amount_out, .. } => {
                if *amount_in == 0 {
                    return Err(DeFiHubError::Internal("Swap input amount cannot be zero".to_string()));
                }
                if *min_amount_out == 0 {
                    return Err(DeFiHubError::Internal("Minimum output amount cannot be zero".to_string()));
                }
            },
            Operation::ProvideLiquidity { amount_a, amount_b, .. } => {
                if *amount_a == 0 || *amount_b == 0 {
                    return Err(DeFiHubError::Internal("Liquidity amounts cannot be zero".to_string()));
                }
            },
        }
        
        Ok(())
    }
    
    /// 배치 내 작업들이 충돌하지 않는지 검증
    pub fn validate_batch_operations(operations: &[Operation]) -> DeFiResult<()> {
        // TODO: 더 정교한 검증 로직 구현
        // - 이중 지출 검사
        // - 잔액 검증
        // - 의존성 검사 등
        
        for operation in operations {
            Self::validate_operation(operation)?;
        }
        
        Ok(())
    }
}