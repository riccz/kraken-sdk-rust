use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

/// - <https://docs.kraken.com/rest/#tag/Account-Data/operation/getExtendedBalance>
/// - <https://api.kraken.com/0/private/BalanceEx>
#[must_use = "Does nothing until you send or execute it"]
pub struct GetExtendedBalanceRequest {
    client: Client,
}

impl GetExtendedBalanceRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        self.client.send_private("/0/private/BalanceEx", None).await
    }

    pub async fn send(self) -> Result<GetExtendedBalanceResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct ExtendedBalance {
    pub balance: String,
    pub credit: Option<String>,
    pub credit_used: Option<String>,
    pub hold_trade: String,
}

pub type GetExtendedBalanceResponse = HashMap<String, ExtendedBalance>;

impl Client {
    pub fn get_extended_balance(&self) -> GetExtendedBalanceRequest {
        GetExtendedBalanceRequest {
            client: self.clone(),
        }
    }
}
