use bitcoin::Amount;

/// DeFi 허브 시스템 상수들

// === Bitcoin Layer 상수 ===
pub const DEFAULT_TIMELOCK_BLOCKS: u16 = 20;
pub const MIN_CONFIRMATION_BLOCKS: u16 = 6;
pub const DUST_AMOUNT: Amount = Amount::from_sat(546);
pub const DEFAULT_FEE_RATE: u64 = 10; // sat/vB

// === 롤업 상수 ===
pub const BATCH_INTERVAL_SECONDS: u64 = 30;
pub const MAX_OPERATIONS_PER_BATCH: usize = 1000;
pub const STATE_ROOT_HISTORY_SIZE: usize = 1000;
pub const ROLLUP_CHALLENGE_PERIOD_BLOCKS: u16 = 144; // ~24시간

// === 브릿지 상수 ===
pub const MIN_BRIDGE_AMOUNT: Amount = Amount::from_sat(10_000); // 0.0001 BTC
pub const MAX_BRIDGE_AMOUNT: Amount = Amount::from_sat(100_000_000); // 1 BTC
pub const BRIDGE_FEE_RATE: f64 = 0.001; // 0.1%

// === DeFi 상수 ===
pub const MIN_LIQUIDITY_AMOUNT: u64 = 1000;
pub const MAX_SLIPPAGE_TOLERANCE: f64 = 0.1; // 10%
pub const SWAP_FEE_RATE: f64 = 0.003; // 0.3%

// === 네트워크 상수 ===
pub const RPC_TIMEOUT_SECONDS: u64 = 30;
pub const MAX_RETRY_ATTEMPTS: u32 = 3;
pub const BLOCK_CONFIRMATION_WAIT: u64 = 10; // seconds

// === 파일 경로 ===
pub const VAULT_STATE_FILE: &str = "vault_state.json";
pub const ROLLUP_STATE_FILE: &str = "rollup_state.json";
pub const BRIDGE_CONFIG_FILE: &str = "bridge_config.json";

// === BitVMX 상수 ===
pub const BITVMX_ELF_PATH: &str = "BitVMX-CPU/bitvmx-programs/vault_condition.elf";
pub const MIN_VERIFIERS: usize = 1;
pub const SIGNATURE_SIZE: usize = 32;

// === 체인별 설정 ===
pub mod chains {
    pub const BITCOIN_NETWORK_MAGIC: u32 = 0xD9B4BEF9;
    pub const SOLANA_CLUSTER: &str = "mainnet-beta";
    pub const ETHEREUM_CHAIN_ID: u64 = 1;
    
    // 각 체인별 최소 컨펌 수
    pub const BITCOIN_MIN_CONFIRMATIONS: u16 = 6;
    pub const SOLANA_MIN_CONFIRMATIONS: u16 = 32;
    pub const ETHEREUM_MIN_CONFIRMATIONS: u16 = 12;
}

// === 테스트 모드 상수 ===
pub const TEST_MODE_ENV_VAR: &str = "PURRFECT_TEST_MODE";
pub const DEMO_PRIVATE_KEY: &str = "cVt4o7BGAig1UXywgGSmARhxMdzP5qvQsxKkSsc1XEkw3tDTQFpy";

// === 로깅 & 모니터링 ===
pub const LOG_LEVEL_ENV_VAR: &str = "PURRFECT_LOG_LEVEL";
pub const METRICS_PORT: u16 = 9090;
pub const HEALTH_CHECK_PORT: u16 = 8080;