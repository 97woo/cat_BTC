use thiserror::Error;

/// DeFi 허브 공통 에러 타입
#[derive(Error, Debug)]
pub enum DeFiHubError {
    // Bitcoin 관련 에러
    #[error("Bitcoin transaction error: {0}")]
    BitcoinTransaction(String),
    
    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u64, available: u64 },
    
    #[error("Invalid Bitcoin address: {0}")]
    InvalidAddress(String),
    
    // 금고 관련 에러
    #[error("Vault not found")]
    VaultNotFound,
    
    #[error("Vault in invalid state: {current}, expected: {expected}")]
    InvalidVaultState { current: String, expected: String },
    
    #[error("Timelock not expired: {blocks_remaining} blocks remaining")]
    TimelockNotExpired { blocks_remaining: u16 },
    
    // 롤업 관련 에러
    #[error("Rollup execution failed: {0}")]
    RollupExecution(String),
    
    #[error("Invalid state root: expected {expected}, got {actual}")]
    InvalidStateRoot { expected: String, actual: String },
    
    #[error("Batch processing failed: {0}")]
    BatchProcessing(String),
    
    // 브릿지 관련 에러
    #[error("Unsupported chain: {0}")]
    UnsupportedChain(String),
    
    #[error("Bridge message verification failed: {0}")]
    BridgeVerification(String),
    
    #[error("Cross-chain transfer failed: {0}")]
    CrossChainTransfer(String),
    
    // 스왑 관련 에러
    #[error("Insufficient liquidity for swap")]
    InsufficientLiquidity,
    
    #[error("Slippage tolerance exceeded: expected {expected}, got {actual}")]
    SlippageExceeded { expected: u64, actual: u64 },
    
    #[error("Invalid token pair: {token_a} / {token_b}")]
    InvalidTokenPair { token_a: String, token_b: String },
    
    // 네트워킹 에러
    #[error("RPC connection failed: {0}")]
    RpcConnection(String),
    
    #[error("Network timeout: {0}")]
    NetworkTimeout(String),
    
    // 일반적인 에러
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result 타입 별칭
pub type DeFiResult<T> = Result<T, DeFiHubError>;

impl From<bitcoin::consensus::encode::Error> for DeFiHubError {
    fn from(err: bitcoin::consensus::encode::Error) -> Self {
        DeFiHubError::BitcoinTransaction(err.to_string())
    }
}

impl From<serde_json::Error> for DeFiHubError {
    fn from(err: serde_json::Error) -> Self {
        DeFiHubError::Serialization(err.to_string())
    }
}

impl From<anyhow::Error> for DeFiHubError {
    fn from(err: anyhow::Error) -> Self {
        DeFiHubError::Internal(err.to_string())
    }
}