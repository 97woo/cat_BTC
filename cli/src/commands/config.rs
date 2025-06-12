use crate::config::Config;
use anyhow::Result;
use tracing::info;

use super::{ConfigCommands};

pub async fn handle_config_command(cmd: ConfigCommands, config_path: &str) -> Result<()> {
    match cmd {
        ConfigCommands::Init => {
            info!("⚙️  설정 파일 초기화");
            let config = Config::default();
            config.save(config_path)?;
            info!("✅ 설정 파일이 생성되었습니다: {}", config_path);
        }
        ConfigCommands::Show => {
            info!("📋 현재 설정:");
            match Config::load(config_path) {
                Ok(config) => {
                    info!("  Bitcoin 네트워크: {:?}", config.bitcoin.network);
                    info!("  RPC 엔드포인트: {}", config.bitcoin.rpc_endpoint);
                    info!("  배치 간격: {}초", config.rollup.batch_interval_seconds);
                    info!("  Solana 엔드포인트: {}", config.bridge.solana_endpoint);
                }
                Err(_) => {
                    info!("❌ 설정 파일을 찾을 수 없습니다. 'config init'를 실행하세요.");
                }
            }
        }
        ConfigCommands::Set { key, value } => {
            info!("🔧 설정 변경: {} = {}", key, value);
            match Config::load(config_path) {
                Ok(mut config) => {
                    config.set_value(&key, &value)?;
                    config.save(config_path)?;
                    info!("✅ 설정이 업데이트되었습니다!");
                }
                Err(_) => {
                    info!("❌ 설정 파일을 찾을 수 없습니다. 'config init'를 먼저 실행하세요.");
                }
            }
        }
    }
    Ok(())
}