//
//  lp_bot.rs
//  marketmaker
//

use common::{mm_ctx::{from_ctx, MmArc},
             mm_number::MmNumber};
use derive_more::Display;
use futures::lock::Mutex as AsyncMutex;
use std::{collections::HashMap, sync::Arc};

#[cfg(test)] use mocktopus::macros::*;

#[path = "lp_bot/simple_market_maker.rs"]
mod simple_market_maker_bot;
pub use simple_market_maker_bot::{process_price_request, start_simple_market_maker_bot, stop_simple_market_maker_bot,
                                  StartSimpleMakerBotRequest};

#[derive(PartialEq)]
enum TradingBotState {
    Running,
    Stopping,
    Stopped,
}

impl Default for TradingBotState {
    fn default() -> Self { TradingBotState::Stopped }
}

pub type SimpleMakerBotRegistry = HashMap<String, SimpleCoinMarketMakerCfg>;

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[display(fmt = "{} {} {} {}", base, rel, min_volume, spread)]
pub struct SimpleCoinMarketMakerCfg {
    base: String,
    rel: String,
    min_volume: MmNumber,
    spread: MmNumber,
    base_confs: u64,
    base_nota: bool,
    rel_confs: u64,
    rel_nota: bool,
    enable: bool,
    price_elapsed_validity: Option<f64>,
    check_last_bidirectional_trade_thresh_hold: Option<bool>,
    max: Option<bool>,
    balance_percent: Option<common::mm_number::MmNumber>,
}

pub type TickerInfosRegistry = HashMap<String, TickerInfos>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TickerInfos {
    ticker: String,
    last_price: String,
    last_updated: String,
    last_updated_timestamp: i64,
    #[serde(rename = "volume24h")]
    volume24_h: String,
    price_provider: Provider,
    volume_provider: Provider,
    #[serde(rename = "sparkline_7d")]
    sparkline_7_d: Option<Vec<f64>>,
    sparkline_provider: Provider,
    #[serde(rename = "change_24h")]
    change_24_h: String,
    #[serde(rename = "change_24h_provider")]
    change_24_h_provider: Provider,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "binance")]
    Binance,
    #[serde(rename = "coingecko")]
    Coingecko,
    #[serde(rename = "coinpaprika")]
    Coinpaprika,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Default)]
struct TradingBotContext {
    pub trading_bot_states: AsyncMutex<TradingBotState>,
    pub trading_bot_cfg: AsyncMutex<SimpleMakerBotRegistry>,
    pub price_tickers_registry: AsyncMutex<TickerInfosRegistry>,
}

#[cfg_attr(test, mockable)]
impl TradingBotContext {
    /// Obtains a reference to this crate context, creating it if necessary.
    fn from_ctx(ctx: &MmArc) -> Result<Arc<TradingBotContext>, String> {
        Ok(try_s!(from_ctx(&ctx.simple_market_maker_bot_ctx, move || {
            Ok(TradingBotContext::default())
        })))
    }
}
