use crate::{StateRoot, VaultState, BatchOperation, BridgeMessage, DeFiResult, TokenType};
use bitcoin::{Amount, OutPoint};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// 전체 DeFi 허브의 글로벌 상태
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GlobalState {
    /// Bitcoin 금고 상태들
    pub vaults: HashMap<OutPoint, VaultInfo>,
    
    /// 롤업 상태
    pub rollup: RollupState,
    
    /// 브릿지 상태
    pub bridge: BridgeState,
    
    /// 마지막 업데이트 시간
    pub last_updated: DateTime<Utc>,
}

/// 금고 정보
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VaultInfo {
    pub outpoint: OutPoint,
    pub amount: Amount,
    pub state: VaultState,
    pub owner: String,
    pub created_at: DateTime<Utc>,
}

/// 롤업 상태
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RollupState {
    /// 현재 상태 루트
    pub current_state_root: StateRoot,
    
    /// 계정 잔액들 (rollup 내 주소 → 토큰 → 잔액)
    pub balances: HashMap<String, HashMap<TokenType, u64>>,
    
    /// 유동성 풀들 (토큰쌍 → 유동성 정보)
    pub liquidity_pools: HashMap<(TokenType, TokenType), LiquidityPool>,
    
    /// 처리된 배치들
    pub processed_batches: Vec<BatchOperation>,
    
    /// 다음 배치 처리 예정 시간
    pub next_batch_time: DateTime<Utc>,
}

/// 유동성 풀 정보
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LiquidityPool {
    pub token_a: TokenType,
    pub token_b: TokenType,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub total_liquidity: u64,
    pub fee_rate: f64,
}

/// 브릿지 상태
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BridgeState {
    /// 체인별 연결 상태
    pub chain_connections: HashMap<String, ChainConnection>,
    
    /// 처리 중인 메시지들
    pub pending_messages: Vec<BridgeMessage>,
    
    /// 완료된 메시지들 (최근 1000개)
    pub completed_messages: Vec<BridgeMessage>,
}

/// 체인 연결 정보
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChainConnection {
    pub chain_id: String,
    pub endpoint: String,
    pub is_connected: bool,
    pub last_block_height: u64,
    pub last_sync: DateTime<Utc>,
}

impl GlobalState {
    /// 새로운 글로벌 상태 생성
    pub fn new() -> Self {
        Self {
            vaults: HashMap::new(),
            rollup: RollupState::new(),
            bridge: BridgeState::new(),
            last_updated: Utc::now(),
        }
    }
    
    /// 상태를 파일에서 로드
    pub fn load_from_file(path: &str) -> DeFiResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| crate::DeFiHubError::Configuration(format!("Failed to read state file: {}", e)))?;
        
        let state: GlobalState = serde_json::from_str(&content)?;
        Ok(state)
    }
    
    /// 상태를 파일에 저장
    pub fn save_to_file(&self, path: &str) -> DeFiResult<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)
            .map_err(|e| crate::DeFiHubError::Configuration(format!("Failed to write state file: {}", e)))?;
        Ok(())
    }
    
    /// 금고 추가
    pub fn add_vault(&mut self, vault_info: VaultInfo) {
        self.vaults.insert(vault_info.outpoint, vault_info);
        self.last_updated = Utc::now();
    }
    
    /// 금고 상태 업데이트
    pub fn update_vault_state(&mut self, outpoint: &OutPoint, new_state: VaultState) -> DeFiResult<()> {
        match self.vaults.get_mut(outpoint) {
            Some(vault) => {
                vault.state = new_state;
                self.last_updated = Utc::now();
                Ok(())
            }
            None => Err(crate::DeFiHubError::VaultNotFound),
        }
    }
    
    /// 롤업 상태 업데이트
    pub fn update_rollup_state(&mut self, new_state_root: StateRoot) {
        self.rollup.current_state_root = new_state_root;
        self.last_updated = Utc::now();
    }
}

impl RollupState {
    pub fn new() -> Self {
        Self {
            current_state_root: StateRoot {
                hash: [0; 32],
                height: 0,
                timestamp: Utc::now(),
            },
            balances: HashMap::new(),
            liquidity_pools: HashMap::new(),
            processed_batches: Vec::new(),
            next_batch_time: Utc::now() + chrono::Duration::seconds(30),
        }
    }
    
    /// 계정 잔액 조회
    pub fn get_balance(&self, address: &str, token: &TokenType) -> u64 {
        self.balances
            .get(address)
            .and_then(|tokens| tokens.get(token))
            .copied()
            .unwrap_or(0)
    }
    
    /// 계정 잔액 설정
    pub fn set_balance(&mut self, address: String, token: TokenType, amount: u64) {
        self.balances
            .entry(address)
            .or_insert_with(HashMap::new)
            .insert(token, amount);
    }
}

impl BridgeState {
    pub fn new() -> Self {
        Self {
            chain_connections: HashMap::new(),
            pending_messages: Vec::new(),
            completed_messages: Vec::new(),
        }
    }
    
    /// 체인 연결 추가
    pub fn add_chain_connection(&mut self, connection: ChainConnection) {
        self.chain_connections.insert(connection.chain_id.clone(), connection);
    }
    
    /// 브릿지 메시지 추가
    pub fn add_pending_message(&mut self, message: BridgeMessage) {
        self.pending_messages.push(message);
    }
    
    /// 메시지 완료 처리
    pub fn complete_message(&mut self, message_id: &uuid::Uuid) -> DeFiResult<()> {
        if let Some(pos) = self.pending_messages.iter().position(|msg| &msg.id == message_id) {
            let completed_message = self.pending_messages.remove(pos);
            self.completed_messages.push(completed_message);
            
            // 최근 1000개만 유지
            if self.completed_messages.len() > 1000 {
                self.completed_messages.remove(0);
            }
            
            Ok(())
        } else {
            Err(crate::DeFiHubError::Internal("Message not found".to_string()))
        }
    }
}