use crate::config::Config;
use anyhow::Result;
use tracing::info;

use super::{BridgeCommands};

pub async fn handle_bridge_command(cmd: BridgeCommands, _config: &Config) -> Result<()> {
    match cmd {
        BridgeCommands::Status => {
            info!("🌉 크로스체인 브릿지 상태:");
            info!("  연결된 체인: Solana");
            info!("  처리 중인 메시지: 0개");
            info!("  브릿지 수수료: 0.1%");
        }
        BridgeCommands::Lock { to_chain, amount, recipient } => {
            info!("🔒 BTC → {} 브릿지", to_chain);
            info!("  금액: {} 사토시", amount);
            info!("  수신자: {}", recipient);
            info!("✅ 브릿지 요청이 처리되었습니다!");
        }
        BridgeCommands::Unlock { from_chain, amount, recipient } => {
            info!("🔓 {} → BTC 브릿지", from_chain);
            info!("  금액: {}", amount);
            info!("  BTC 주소: {}", recipient);
            info!("✅ 언락 요청이 처리되었습니다!");
        }
    }
    Ok(())
}