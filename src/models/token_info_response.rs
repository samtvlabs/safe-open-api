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
pub struct TokenInfoResponse {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "decimals")]
    pub decimals: i32,
    #[serde(rename = "logoUri", skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
}

impl TokenInfoResponse {
    pub fn new(address: String, name: String, symbol: String, decimals: i32) -> TokenInfoResponse {
        TokenInfoResponse {
            r#type: None,
            address,
            name,
            symbol,
            decimals,
            logo_uri: None,
        }
    }
}

