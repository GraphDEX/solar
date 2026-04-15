use std::str::FromStr;

use eyre::{Context, Result};
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

#[cfg(feature = "encryptor")]
use crate::encryptor::{EncryptionConfig, Encryptor};

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Hash, Copy)]
#[cfg_attr(feature = "axum", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum Chain {
    Solana,
}

impl TryFrom<String> for Chain {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&format!(r#""{}""#, value))
    }
}

impl std::fmt::Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self)
                .expect("failed to serialize chain")
                .trim_matches('"')
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Hash, Copy)]
#[cfg_attr(feature = "axum", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum Dex {
    // Pump.fun
    #[serde(alias = "Pump.fun", alias = "pump.fun", alias = "Pumpfun")]
    Pumpfun,
    #[serde(alias = "Pump.fun AMM", alias = "pump.fun AMM", alias = "PumpAmm")]
    PumpAmm,
    // Raydium
    #[serde(alias = "Raydium AMM", alias = "raydium AMM", alias = "RaydiumAmm")]
    RaydiumAmm,
    #[serde(alias = "Raydium CLMM", alias = "raydium CLMM", alias = "RaydiumClmm")]
    RaydiumClmm,
    #[serde(alias = "Raydium CPMM", alias = "raydium CPMM", alias = "RaydiumCpmm")]
    RaydiumCpmm,
    #[serde(alias = "BONKfun", alias = "bonkfun", alias = "BonkFun")]
    BonkFun,
    // Meteora
    #[serde(alias = "Meteora DLMM", alias = "meteora DLMM", alias = "MeteoraDlmm")]
    MeteoraDlmm,
    #[serde(
        alias = "Meteora DAMM",
        alias = "Meteora DAMM V1",
        alias = "meteora_damm"
    )]
    MeteoraDammV1,
    #[serde(
        alias = "Meteora DAMM V2",
        alias = "Meteora damm V2",
        alias = "meteora damm v2"
    )]
    MeteoraDammV2,
    MeteoraDbc,
    // Orca
    #[serde(
        alias = "Orca Whirlpool",
        alias = "orca whirlpool",
        alias = "OrcaWhirlpool",
        alias = "Orca",
        alias = "orca"
    )]
    OrcaWhirlpool,
    BoopFun,
    Moonshot,
}

impl Dex {
    pub fn to_label(&self) -> String {
        match self {
            Dex::Pumpfun => "Pumpfun".to_string(),
            Dex::PumpAmm => "Pump.fun AMM".to_string(),

            Dex::RaydiumAmm => "Raydium AMM".to_string(),
            Dex::RaydiumClmm => "Raydium CLMM".to_string(),
            Dex::RaydiumCpmm => "Raydium CPMM".to_string(),

            Dex::MeteoraDlmm => "Meteora DLMM".to_string(),
            Dex::MeteoraDammV1 => "Meteora DAMM V1".to_string(),
            Dex::MeteoraDammV2 => "Meteora DAMM V2".to_string(),
            Dex::MeteoraDbc => "Meteora DBC".to_string(),

            Dex::OrcaWhirlpool => "Orca Whirlpool".to_string(),

            Dex::BonkFun => "BONKfun".to_string(),
            Dex::BoopFun => "boop.fun".to_string(),
            Dex::Moonshot => "Moonshot".to_string(),
        }
    }
}

impl TryFrom<String> for Dex {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&format!(r#""{}""#, value))
    }
}

impl std::fmt::Display for Dex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self)
                .expect("failed to serialize dex")
                .trim_matches('"')
        )
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "axum", derive(utoipa::ToSchema))]
#[serde(transparent)]
#[readonly::make]
pub struct Address {
    pub value: String,
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<String> for Address {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<&str> for Address {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

#[cfg(feature = "solana")]
impl From<&Pubkey> for Address {
    fn from(value: &Pubkey) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl Address {
    #[cfg(feature = "solana")]
    pub fn pubkey(&self) -> eyre::Result<Pubkey> {
        Pubkey::from_str(&self.value).context("failed to parse pubkey")
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "axum", derive(utoipa::ToSchema))]
pub struct PrivateKey {
    pub value: String,
}

impl From<String> for PrivateKey {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<&str> for PrivateKey {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

#[cfg(feature = "solana")]
impl From<&Keypair> for PrivateKey {
    fn from(value: &Keypair) -> Self {
        Self {
            value: value.to_base58_string(),
        }
    }
}

impl std::fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PrivateKey {
    #[cfg(feature = "encryptor")]
    pub fn encrypt(&self, config: &EncryptionConfig) -> Result<PrivateKeyEncrypted> {
        let encryptor = Encryptor::new(config);
        let encrypted = encryptor.encrypt(&self.value)?;
        Ok(PrivateKeyEncrypted { value: encrypted })
    }

    #[cfg(feature = "solana")]
    pub fn keypair(&self) -> eyre::Result<Keypair> {
        use solana_sdk::bs58;

        let bytes = bs58::decode(&self.value)
            .into_vec()
            .context("invalid base58 private key encoding")?;
        Keypair::from_bytes(bytes.as_ref()).context("invalid private key")
    }

    #[cfg(feature = "solana")]
    pub fn pubkey(&self) -> eyre::Result<Pubkey> {
        Ok(self.keypair()?.pubkey())
    }
}

#[cfg(feature = "encryptor")]
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrivateKeyEncrypted {
    pub value: String,
}

#[cfg(feature = "encryptor")]
impl From<String> for PrivateKeyEncrypted {
    fn from(value: String) -> Self {
        Self { value }
    }
}

#[cfg(feature = "encryptor")]
impl PrivateKeyEncrypted {
    pub fn decrypt(&self, config: &EncryptionConfig) -> Result<PrivateKey> {
        let encryptor = Encryptor::new(config);
        let decrypted = encryptor.decrypt(&self.value)?;
        Ok(PrivateKey { value: decrypted })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "axum", derive(utoipa::ToSchema))]
#[serde(transparent)]
#[readonly::make]
pub struct TransactionHash {
    pub value: String,
}

impl From<String> for TransactionHash {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<&str> for TransactionHash {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl std::fmt::Display for TransactionHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
