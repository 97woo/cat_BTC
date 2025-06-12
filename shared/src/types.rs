use bitcoin::{Amount, OutPoint};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// DeFi 허브의 핵심 상태 타입들

/// BTC 금고 상태
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VaultState {
    /// 비활성 상태 - 출금 가능
    Inactive,
    /// 출금 트리거됨 - 타임락 대기 중
    Triggered {
        withdrawal_address: String,
        amount: Amount,
        trigger_time: DateTime<Utc>,
        timelock_blocks: u16,
    },
    /// 출금 완료
    Completed,
    /// BitVMX 롤업과 연결된 상태
    Bridged {
        rollup_state_root: StateRoot,
        last_sync: DateTime<Utc>,
    },
}

/// 미니 롤업 배치 처리
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BatchOperation {
    pub id: Uuid,
    pub operations: Vec<Operation>,
    pub timestamp: DateTime<Utc>,
    pub previous_state_root: StateRoot,
    pub new_state_root: StateRoot,
    pub signature: Option<Vec<u8>>,
}

/// DeFi 작업 타입들
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Operation {
    /// BTC 예치 (L1 → 롤업)
    Deposit {
        vault_outpoint: OutPoint,
        amount: Amount,
        recipient: String, // 롤업 내 주소
    },
    /// BTC 출금 (롤업 → L1)
    Withdraw {
        rollup_address: String,
        amount: Amount,
        destination: String, // L1 주소
    },
    /// 롤업 내 스왑
    Swap {
        from_token: TokenType,
        to_token: TokenType,
        amount_in: u64,
        min_amount_out: u64,
        user: String,
    },
    /// 유동성 공급
    ProvideLiquidity {
        token_a: TokenType,
        token_b: TokenType,
        amount_a: u64,
        amount_b: u64,
        provider: String,
    },
}

/// 토큰 타입 (롤업 내)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    /// 래핑된 BTC
    WBTC,
    /// 스테이블코인
    USDC,
    /// 기타 토큰
    Custom(String),
}

/// 상태 루트
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StateRoot {
    pub hash: [u8; 32],
    pub height: u64,
    pub timestamp: DateTime<Utc>,
}

/// 크로스체인 브릿지 메시지
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BridgeMessage {
    pub id: Uuid,
    pub from_chain: ChainId,
    pub to_chain: ChainId,
    pub operation: BridgeOperation,
    pub nonce: u64,
    pub timestamp: DateTime<Utc>,
}

/// 지원되는 체인들
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ChainId {
    Bitcoin,
    FractalBitcoin,
    Solana,
    Ethereum,
    Polygon,
    Custom(String),
}

/// 브릿지 작업 타입
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BridgeOperation {
    /// 토큰 락 (원본 체인)
    Lock {
        token: String,
        amount: u64,
        recipient: String,
    },
    /// 토큰 민트 (대상 체인)  
    Mint {
        token: String,
        amount: u64,
        recipient: String,
    },
    /// 토큰 번 (원본 체인)
    Burn {
        token: String,
        amount: u64,
        recipient: String,
    },
    /// 토큰 언락 (대상 체인)
    Unlock {
        token: String,
        amount: u64,
        recipient: String,
    },
}

/// 롤업 실행 결과
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExecutionResult {
    pub success: bool,
    pub gas_used: u64,
    pub new_state_root: StateRoot,
    pub events: Vec<Event>,
    pub error: Option<String>,
}

/// 이벤트 타입
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    Transfer {
        from: String,
        to: String,
        token: TokenType,
        amount: u64,
    },
    Swap {
        user: String,
        token_in: TokenType,
        token_out: TokenType,
        amount_in: u64,
        amount_out: u64,
    },
    Deposit {
        user: String,
        amount: Amount,
        rollup_address: String,
    },
    Withdrawal {
        user: String,
        amount: Amount,
        bitcoin_address: String,
    },
}