use bitcoin::Network;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::Path;

/// 통합 DeFi 허브 설정
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    /// Bitcoin 네트워크 설정
    pub bitcoin: BitcoinConfig,
    
    /// Fractal Bitcoin 설정
    pub fractal: FractalBitcoinConfig,
    
    /// 롤업 설정
    pub rollup: RollupConfig,
    
    /// 브릿지 설정
    pub bridge: BridgeConfig,
    
    /// DeFi 설정
    pub defi: DefiConfig,
    
    /// 시스템 설정
    pub system: SystemConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BitcoinConfig {
    /// 비트코인 네트워크
    pub network: Network,
    
    /// RPC 설정
    pub rpc_endpoint: String,
    pub rpc_username: String,
    pub rpc_password: String,
    
    /// 금고 설정
    pub default_timelock_blocks: u16,
    pub vault_state_file: String,
    
    /// BitVMX 설정
    pub bitvmx_elf_path: String,
}

/// Fractal Bitcoin 설정
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FractalBitcoinConfig {
    /// Fractal 네트워크 활성화
    pub enabled: bool,
    
    /// Fractal RPC 설정
    pub rpc_endpoint: String,
    pub rpc_username: String,
    pub rpc_password: String,
    
    /// Fractal 체인 ID
    pub chain_id: u32,
    
    /// 블록 시간 (초)
    pub block_time: u64,
    
    /// OP_CAT 지원 여부
    pub op_cat_enabled: bool,
    
    /// 금고 설정
    pub vault_address_prefix: String,
    pub default_timelock_blocks: u16,
    
    /// Fractal-Bitcoin 브릿지 설정
    pub bridge_contract_address: Option<String>,
    pub min_confirmations: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RollupConfig {
    /// 배치 처리 간격 (초)
    pub batch_interval_seconds: u64,
    
    /// 최대 배치 크기
    pub max_batch_size: usize,
    
    /// 상태 파일
    pub state_file: String,
    
    /// 자동 시작 여부
    pub auto_start: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BridgeConfig {
    /// Solana 설정
    pub solana_endpoint: String,
    pub solana_program_id: String,
    
    /// Ethereum 설정 (선택사항)
    pub ethereum_endpoint: Option<String>,
    pub ethereum_contract_address: Option<String>,
    
    /// 브릿지 수수료율
    pub fee_rate: f64,
    
    /// 최소/최대 브릿지 금액
    pub min_bridge_amount: u64,
    pub max_bridge_amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DefiConfig {
    /// 스왑 수수료율
    pub swap_fee_rate: f64,
    
    /// 최대 슬리피지 허용치
    pub max_slippage: f64,
    
    /// 최소 유동성 금액
    pub min_liquidity_amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SystemConfig {
    /// 로그 레벨
    pub log_level: String,
    
    /// 데이터 디렉토리
    pub data_dir: String,
    
    /// API 서버 포트
    pub api_port: u16,
    
    /// 메트릭스 포트
    pub metrics_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bitcoin: BitcoinConfig {
                network: Network::Regtest,
                rpc_endpoint: "http://127.0.0.1:18443".to_string(),
                rpc_username: "user".to_string(),
                rpc_password: "pass".to_string(),
                default_timelock_blocks: 20,
                vault_state_file: "vault_state.json".to_string(),
                bitvmx_elf_path: "BitVMX-CPU/bitvmx-programs/vault_condition.elf".to_string(),
            },
            fractal: FractalBitcoinConfig {
                enabled: true,
                rpc_endpoint: "https://open-api-fractal.unisat.io".to_string(),
                rpc_username: "".to_string(),
                rpc_password: "".to_string(),
                chain_id: 1,
                block_time: 30, // Fractal은 30초 블록  
                op_cat_enabled: true,
                vault_address_prefix: "bc1".to_string(),
                default_timelock_blocks: 6, // 30초 * 6 = 3분
                bridge_contract_address: None,
                min_confirmations: 6,
            },
            rollup: RollupConfig {
                batch_interval_seconds: 30,
                max_batch_size: 1000,
                state_file: "rollup_state.json".to_string(),
                auto_start: true,
            },
            bridge: BridgeConfig {
                solana_endpoint: "https://api.devnet.solana.com".to_string(),
                solana_program_id: "11111111111111111111111111111111".to_string(),
                ethereum_endpoint: None,
                ethereum_contract_address: None,
                fee_rate: 0.001, // 0.1%
                min_bridge_amount: 10_000, // 0.0001 BTC
                max_bridge_amount: 100_000_000, // 1 BTC
            },
            defi: DefiConfig {
                swap_fee_rate: 0.003, // 0.3%
                max_slippage: 0.1, // 10%
                min_liquidity_amount: 1000,
            },
            system: SystemConfig {
                log_level: "info".to_string(),
                data_dir: "./data".to_string(),
                api_port: 8080,
                metrics_port: 9090,
            },
        }
    }
}

impl Config {
    /// 설정 파일에서 로드
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    
    /// 설정 파일에 저장
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// 데이터 디렉토리 생성
    pub fn ensure_data_dir(&self) -> Result<()> {
        std::fs::create_dir_all(&self.system.data_dir)?;
        Ok(())
    }
    
    /// 설정 값 업데이트
    pub fn set_value(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "bitcoin.network" => {
                self.bitcoin.network = match value.to_lowercase().as_str() {
                    "mainnet" => Network::Bitcoin,
                    "testnet" => Network::Testnet,
                    "regtest" => Network::Regtest,
                    "signet" => Network::Signet,
                    _ => return Err(anyhow::anyhow!("Invalid network: {}", value)),
                };
            },
            "bitcoin.rpc_endpoint" => self.bitcoin.rpc_endpoint = value.to_string(),
            "bitcoin.rpc_username" => self.bitcoin.rpc_username = value.to_string(),
            "bitcoin.rpc_password" => self.bitcoin.rpc_password = value.to_string(),
            "bitcoin.default_timelock_blocks" => {
                self.bitcoin.default_timelock_blocks = value.parse()?;
            },
            "rollup.batch_interval_seconds" => {
                self.rollup.batch_interval_seconds = value.parse()?;
            },
            "rollup.max_batch_size" => {
                self.rollup.max_batch_size = value.parse()?;
            },
            "rollup.auto_start" => {
                self.rollup.auto_start = value.parse()?;
            },
            "bridge.solana_endpoint" => self.bridge.solana_endpoint = value.to_string(),
            "bridge.fee_rate" => {
                self.bridge.fee_rate = value.parse()?;
            },
            "defi.swap_fee_rate" => {
                self.defi.swap_fee_rate = value.parse()?;
            },
            "defi.max_slippage" => {
                self.defi.max_slippage = value.parse()?;
            },
            "system.log_level" => self.system.log_level = value.to_string(),
            "system.data_dir" => self.system.data_dir = value.to_string(),
            "system.api_port" => {
                self.system.api_port = value.parse()?;
            },
            _ => return Err(anyhow::anyhow!("Unknown config key: {}", key)),
        }
        Ok(())
    }
}