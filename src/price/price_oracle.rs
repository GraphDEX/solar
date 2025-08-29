use std::sync::Arc;
use std::time::Duration;

use eyre::Context;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use spl_token::state::Account;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

use crate::consts::{
    SOL_DECIMALS, SOL_USDC_POOL_SOL_VAULT, SOL_USDC_POOL_USDC_VAULT, USDC_DECIMALS,
};
use crate::tool::from_u64;

#[derive(thiserror::Error, Debug)]
pub enum PriceOracleError {
    #[error("internal error: {0:?}")]
    InternalError(#[from] eyre::Error),
}

#[async_trait::async_trait]
pub trait PriceOracle {
    async fn get_sol_usd_price(&self) -> Result<f64, PriceOracleError>;
}

const SOL_VAULT_ACCOUNT: Pubkey = Pubkey::from_str_const(SOL_USDC_POOL_SOL_VAULT);
const USDC_VAULT_ACCOUNT: Pubkey = Pubkey::from_str_const(SOL_USDC_POOL_USDC_VAULT);

pub struct NativePriceOracleBuilder {
    solana_rpc_url: String,
    update_interval: Duration,
}

impl NativePriceOracleBuilder {
    pub fn new(solana_rpc_url: impl Into<String>, update_interval: Duration) -> Self {
        Self {
            solana_rpc_url: solana_rpc_url.into(),
            update_interval,
        }
    }

    pub async fn build(self) -> Result<NativePriceOracle, PriceOracleError> {
        let price_oracle = NativePriceOracle::new(self.solana_rpc_url, self.update_interval);
        price_oracle.prepare().await?;
        Ok(price_oracle)
    }
}

pub struct NativePriceOracle {
    solana_rpc_url: String,
    update_interval: Duration,
    sol_usd_price: RwLock<f64>,
}

impl NativePriceOracle {
    fn new(solana_rpc_url: impl Into<String>, update_interval: Duration) -> Self {
        Self {
            solana_rpc_url: solana_rpc_url.into(),
            update_interval,
            sol_usd_price: RwLock::new(0.0),
        }
    }

    pub async fn run(
        self: Arc<Self>,
        cancel_token: CancellationToken,
    ) -> Result<(), PriceOracleError> {
        let mut interval = tokio::time::interval(self.update_interval);
        let rpc_client = RpcClient::new(self.solana_rpc_url.clone());

        // First tick returns immediately, so skip it
        interval.tick().await;

        loop {
            tokio::select! {
                _ = self.try_update_sol_usd_price(&mut interval, &rpc_client) => {}
                _ = cancel_token.cancelled() => {

                    #[cfg(feature = "log")]
                    log::info!(client = "NativePriceOracle"; "stopped");

                    return Ok(());
                }
            }
        }
    }

    async fn try_update_sol_usd_price(self: &Arc<Self>, interval: &mut tokio::time::Interval, rpc_client: &RpcClient) {
        interval.tick().await;
        match Self::get_sol_usd_price_native(rpc_client).await {
            Ok(price) => {
                let mut sol_usd_price = self.sol_usd_price.write().await;
                *sol_usd_price = price;
            },
            Err(_err) => {
                #[cfg(feature = "log")]
                log::error!(client = "NativePriceOracle"; "failed to get price: {_err:?}");
            }
        };
    }

    async fn prepare(&self) -> Result<(), PriceOracleError> {
        let rpc_url = self.solana_rpc_url.clone();
        let rpc_client = RpcClient::new(rpc_url);

        let price = Self::get_sol_usd_price_native(&rpc_client)
            .await
            .context("failed to get price")?;

        {
            let mut sol_usd_price = self.sol_usd_price.write().await;
            *sol_usd_price = price;
        }

        Ok(())
    }

    async fn get_sol_usd_price_native(rpc_client: &RpcClient) -> Result<f64, PriceOracleError> {
        let sol_token_account = rpc_client
            .get_account(&SOL_VAULT_ACCOUNT)
            .await
            .context("failed to fetch USDC vault account")?;
        let sol_token_account = Account::unpack(&sol_token_account.data)
            .context("failed to unpack SOL vault account")?;

        let usdc_token_account = rpc_client
            .get_account(&USDC_VAULT_ACCOUNT)
            .await
            .context("failed to fetch SOL vault account")?;
        let usdc_token_account = Account::unpack(&usdc_token_account.data)
            .context("failed to unpack USDC vault account")?;

        let sol_balance = from_u64(sol_token_account.amount, SOL_DECIMALS);
        let usdc_balance = from_u64(usdc_token_account.amount, USDC_DECIMALS);
        let price = usdc_balance / sol_balance;

        Ok(price)
    }
}

#[async_trait::async_trait]
impl PriceOracle for NativePriceOracle {
    async fn get_sol_usd_price(&self) -> Result<f64, PriceOracleError> {
        let sol_usd_price = self.sol_usd_price.read().await;
        Ok(*sol_usd_price)
    }
}

#[async_trait::async_trait]
impl PriceOracle for Arc<NativePriceOracle> {
    async fn get_sol_usd_price(&self) -> Result<f64, PriceOracleError> {
        let sol_usd_price = self.sol_usd_price.read().await;
        Ok(*sol_usd_price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RPC_URL: &str = "https://rpc.ankr.com/solana/055fc89e9d461d5836c0dd01a50c999323c12b4d5b073690925ed92c5c677e5a";

    #[tokio::test]
    async fn test_get_sol_usd_price() {
        let builder = NativePriceOracleBuilder::new(RPC_URL, Duration::from_secs(1));
        let oracle = Arc::new(builder.build().await.unwrap());
        tokio::spawn(oracle.clone().run(CancellationToken::new()));
        let price = oracle.get_sol_usd_price().await.unwrap();
        assert!(price > 0.0);
    }
}
