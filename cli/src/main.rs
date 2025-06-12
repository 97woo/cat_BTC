use clap::{Parser, Subcommand};
use tracing::{info, error};
use anyhow::Result;

mod commands;
mod config;

use commands::*;
use config::Config;

/// Purrfect DeFi Hub - Cross-chain Bitcoin DeFi Platform
#[derive(Parser)]
#[command(
    name = "purrfect",
    version = "0.1.0",
    about = "Cross-chain Bitcoin DeFi Hub with OP_CAT covenants and BitVMX mini-rollup",
    long_about = None
)]
struct Cli {
    /// 설정 파일 경로
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// 로그 레벨
    #[arg(short, long, default_value = "info")]
    log_level: String,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Bitcoin 금고 관리
    #[command(subcommand)]
    Vault(VaultCommands),
    
    /// 미니 롤업 관리
    #[command(subcommand)]
    Rollup(RollupCommands),
    
    /// 크로스체인 브릿지 관리
    #[command(subcommand)]
    Bridge(BridgeCommands),
    
    /// DeFi 작업 (스왑, 유동성 등)
    #[command(subcommand)]
    Defi(DefiCommands),
    
    /// 전체 시스템 상태 조회
    Status,
    
    /// 설정 관리
    #[command(subcommand)]
    Config(ConfigCommands),
}

#[derive(Subcommand)]
enum VaultCommands {
    /// 새 금고 생성
    Create {
        /// 타임락 블록 수
        #[arg(short, long, default_value = "20")]
        timelock: u16,
        
        /// 금고 소유자
        #[arg(short, long)]
        owner: String,
    },
    
    /// BTC 예치
    Deposit {
        /// 예치할 금액 (사토시)
        #[arg(short, long)]
        amount: u64,
    },
    
    /// 출금 트리거
    Trigger {
        /// 출금 주소
        #[arg(short, long)]
        destination: String,
        
        /// 출금 금액 (사토시)
        #[arg(short, long)]
        amount: u64,
    },
    
    /// 출금 완료
    Complete,
    
    /// 출금 취소
    Cancel,
    
    /// 금고 상태 조회
    Status,
    
    /// BitVMX 연동 활성화
    EnableBitvmx {
        /// ELF 프로그램 경로
        #[arg(short, long)]
        elf_path: String,
        
        /// 최소 검증자 수
        #[arg(short, long, default_value = "1")]
        min_verifiers: usize,
    },
}

#[derive(Subcommand)]
enum RollupCommands {
    /// 롤업 시작
    Start,
    
    /// 롤업 중지
    Stop,
    
    /// 배치 처리 상태
    Batch,
    
    /// 롤업 상태 조회
    Status,
    
    /// 계정 잔액 조회
    Balance {
        /// 롤업 주소
        #[arg(short, long)]
        address: String,
        
        /// 토큰 타입 (선택사항)
        #[arg(short, long)]
        token: Option<String>,
    },
}

#[derive(Subcommand)]
enum BridgeCommands {
    /// 브릿지 상태 조회
    Status,
    
    /// BTC → 다른 체인으로 브릿지
    Lock {
        /// 대상 체인
        #[arg(short, long)]
        to_chain: String,
        
        /// 브릿지할 금액 (사토시)
        #[arg(short, long)]
        amount: u64,
        
        /// 수신자 주소
        #[arg(short, long)]
        recipient: String,
    },
    
    /// 다른 체인 → BTC로 브릿지
    Unlock {
        /// 원본 체인
        #[arg(short, long)]
        from_chain: String,
        
        /// 브릿지할 금액
        #[arg(short, long)]
        amount: u64,
        
        /// BTC 수신 주소
        #[arg(short, long)]
        recipient: String,
    },
}

#[derive(Subcommand)]
enum DefiCommands {
    /// 토큰 스왑
    Swap {
        /// 입력 토큰
        #[arg(short, long)]
        from: String,
        
        /// 출력 토큰
        #[arg(short, long)]
        to: String,
        
        /// 입력 금액
        #[arg(short, long)]
        amount: u64,
        
        /// 최소 출력 금액
        #[arg(short, long)]
        min_out: u64,
    },
    
    /// 유동성 공급
    ProvideLiquidity {
        /// 토큰 A
        #[arg(long)]
        token_a: String,
        
        /// 토큰 B
        #[arg(long)]
        token_b: String,
        
        /// 토큰 A 금액
        #[arg(long)]
        amount_a: u64,
        
        /// 토큰 B 금액
        #[arg(long)]
        amount_b: u64,
    },
    
    /// 유동성 제거
    RemoveLiquidity {
        /// 토큰 A
        #[arg(long)]
        token_a: String,
        
        /// 토큰 B
        #[arg(long)]
        token_b: String,
        
        /// 제거할 유동성 양
        #[arg(long)]
        liquidity: u64,
    },
    
    /// 풀 정보 조회
    Pool {
        /// 토큰 A
        #[arg(long)]
        token_a: String,
        
        /// 토큰 B
        #[arg(long)]
        token_b: String,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// 설정 파일 생성
    Init,
    
    /// 현재 설정 표시
    Show,
    
    /// 설정 값 변경
    Set {
        /// 설정 키
        key: String,
        
        /// 설정 값
        value: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // 로깅 초기화
    tracing_subscriber::fmt()
        .with_env_filter(cli.log_level.as_str())
        .init();
    
    // 설정 로드
    let config = Config::load(&cli.config).unwrap_or_else(|_| {
        info!("설정 파일을 찾을 수 없습니다. 기본 설정을 사용합니다.");
        Config::default()
    });
    
    info!("Purrfect DeFi Hub 시작");
    
    // 명령어 실행
    match cli.command {
        Commands::Vault(cmd) => handle_vault_command(cmd, &config).await?,
        Commands::Rollup(cmd) => handle_rollup_command(cmd, &config).await?,
        Commands::Bridge(cmd) => handle_bridge_command(cmd, &config).await?,
        Commands::Defi(cmd) => handle_defi_command(cmd, &config).await?,
        Commands::Status => handle_status_command(&config).await?,
        Commands::Config(cmd) => handle_config_command(cmd, &cli.config).await?,
    }
    
    Ok(())
}