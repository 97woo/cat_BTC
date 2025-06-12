use shared::{VaultState, StateRoot, DeFiResult, DeFiHubError};
use bitcoin::{Address, Amount, OutPoint, Transaction, Network};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::path::Path;

/// Bitcoin 금고 구조체 - DeFi 허브의 핵심 컴포넌트
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BitcoinVault {
    /// 금고 ID (OutPoint)
    pub id: OutPoint,
    
    /// 현재 금고 상태
    pub state: VaultState,
    
    /// 금고 주소
    pub address: Address,
    
    /// 금고 잔액
    pub amount: Amount,
    
    /// 타임락 블록 수
    pub timelock_blocks: u16,
    
    /// 소유자
    pub owner: String,
    
    /// 생성 시간
    pub created_at: DateTime<Utc>,
    
    /// 마지막 업데이트 시간
    pub updated_at: DateTime<Utc>,
    
    /// BitVMX 연동 설정
    pub bitvmx_config: Option<BitVMXConfig>,
}

/// BitVMX 설정
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BitVMXConfig {
    /// ELF 프로그램 경로
    pub elf_path: String,
    
    /// 최소 검증자 수
    pub min_verifiers: usize,
    
    /// 현재 상태 루트
    pub current_state_root: Option<StateRoot>,
    
    /// 마지막 동기화 시간
    pub last_sync: DateTime<Utc>,
}

impl BitcoinVault {
    /// 새로운 금고 생성
    pub fn new(
        network: Network,
        timelock_blocks: u16,
        owner: String,
    ) -> DeFiResult<Self> {
        let now = Utc::now();
        
        // 임시 주소 생성 (실제로는 커버넌트 스크립트에서 생성)
        let address = Self::generate_vault_address(network)?;
        
        Ok(Self {
            id: OutPoint::null(), // 실제 UTXO가 생성되면 업데이트
            state: VaultState::Inactive,
            address,
            amount: Amount::ZERO,
            timelock_blocks,
            owner,
            created_at: now,
            updated_at: now,
            bitvmx_config: None,
        })
    }
    
    /// BitVMX와 연동
    pub fn enable_bitvmx(&mut self, elf_path: String, min_verifiers: usize) {
        self.bitvmx_config = Some(BitVMXConfig {
            elf_path,
            min_verifiers,
            current_state_root: None,
            last_sync: Utc::now(),
        });
        self.updated_at = Utc::now();
    }
    
    /// 금고 상태 업데이트
    pub fn update_state(&mut self, new_state: VaultState) {
        self.state = new_state;
        self.updated_at = Utc::now();
    }
    
    /// 출금 트리거
    pub fn trigger_withdrawal(
        &mut self, 
        withdrawal_address: Address, 
        amount: Amount
    ) -> DeFiResult<()> {
        match &self.state {
            VaultState::Inactive => {
                if amount > self.amount {
                    return Err(DeFiHubError::InsufficientFunds {
                        required: amount.to_sat(),
                        available: self.amount.to_sat(),
                    });
                }
                
                self.state = VaultState::Triggered {
                    withdrawal_address,
                    amount,
                    trigger_time: Utc::now(),
                    timelock_blocks: self.timelock_blocks,
                };
                self.updated_at = Utc::now();
                Ok(())
            },
            _ => Err(DeFiHubError::InvalidVaultState {
                current: format!("{:?}", self.state),
                expected: "Inactive".to_string(),
            }),
        }
    }
    
    /// 출금 완료
    pub fn complete_withdrawal(&mut self) -> DeFiResult<()> {
        match &self.state {
            VaultState::Triggered { .. } => {
                // TODO: 타임락 확인
                self.state = VaultState::Completed;
                self.updated_at = Utc::now();
                Ok(())
            },
            _ => Err(DeFiHubError::InvalidVaultState {
                current: format!("{:?}", self.state),
                expected: "Triggered".to_string(),
            }),
        }
    }
    
    /// 출금 취소
    pub fn cancel_withdrawal(&mut self) -> DeFiResult<()> {
        match &self.state {
            VaultState::Triggered { .. } => {
                self.state = VaultState::Inactive;
                self.updated_at = Utc::now();
                Ok(())
            },
            _ => Err(DeFiHubError::InvalidVaultState {
                current: format!("{:?}", self.state),
                expected: "Triggered".to_string(),
            }),
        }
    }
    
    /// 롤업과 연동
    pub fn bridge_to_rollup(&mut self, state_root: StateRoot) -> DeFiResult<()> {
        self.state = VaultState::Bridged {
            rollup_state_root: state_root,
            last_sync: Utc::now(),
        };
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// 파일에서 로드
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> DeFiResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| DeFiHubError::Configuration(format!("Failed to read vault file: {}", e)))?;
        
        let vault: BitcoinVault = serde_json::from_str(&content)?;
        Ok(vault)
    }
    
    /// 파일에 저장
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> DeFiResult<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)
            .map_err(|e| DeFiHubError::Configuration(format!("Failed to write vault file: {}", e)))?;
        Ok(())
    }
    
    /// 금고 주소 생성 (임시 구현)
    fn generate_vault_address(network: Network) -> DeFiResult<Address> {
        // 임시로 더미 주소 생성
        let dummy_pubkey_bytes = [2u8; 33]; // 압축 공개키
        let pubkey = bitcoin::PublicKey::from_slice(&dummy_pubkey_bytes)
            .map_err(|e| DeFiHubError::BitcoinTransaction(e.to_string()))?;
        
        Ok(Address::p2pkh(&pubkey, network))
    }
    
    /// BitVMX 상태 루트 업데이트
    pub fn update_bitvmx_state(&mut self, state_root: StateRoot) -> DeFiResult<()> {
        match &mut self.bitvmx_config {
            Some(config) => {
                config.current_state_root = Some(state_root);
                config.last_sync = Utc::now();
                self.updated_at = Utc::now();
                Ok(())
            },
            None => Err(DeFiHubError::Configuration("BitVMX not enabled".to_string())),
        }
    }
    
    /// 금고가 활성 상태인지 확인
    pub fn is_active(&self) -> bool {
        !matches!(self.state, VaultState::Completed)
    }
    
    /// 출금 가능 여부 확인
    pub fn can_withdraw(&self) -> bool {
        matches!(self.state, VaultState::Inactive | VaultState::Bridged { .. })
    }
}