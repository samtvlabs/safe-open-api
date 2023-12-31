/*
 * Safe Transaction Service API
 *
 * API to keep track of transactions sent via Gnosis Safe smart contracts
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferWithTokenInfoResponse {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "executionDate")]
    pub execution_date: String,
    #[serde(rename = "blockNumber")]
    pub block_number: i32,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "value", deserialize_with = "Option::deserialize")]
    pub value: Option<String>,
    #[serde(rename = "tokenId", deserialize_with = "Option::deserialize")]
    pub token_id: Option<String>,
    #[serde(rename = "tokenAddress", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub token_address: Option<Option<String>>,
    #[serde(rename = "transferId", skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    #[serde(rename = "tokenInfo")]
    pub token_info: Box<crate::models::TokenInfoResponse>,
    #[serde(rename = "from")]
    pub from: String,
}

impl TransferWithTokenInfoResponse {
    pub fn new(execution_date: String, block_number: i32, transaction_hash: String, to: String, value: Option<String>, token_id: Option<String>, token_info: crate::models::TokenInfoResponse, from: String) -> TransferWithTokenInfoResponse {
        TransferWithTokenInfoResponse {
            r#type: None,
            execution_date,
            block_number,
            transaction_hash,
            to,
            value,
            token_id,
            token_address: None,
            transfer_id: None,
            token_info: Box::new(token_info),
            from,
        }
    }
}


