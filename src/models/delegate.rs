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
pub struct Delegate {
    #[serde(rename = "safe", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub safe: Option<Option<String>>,
    #[serde(rename = "delegate")]
    pub delegate: String,
    #[serde(rename = "delegator")]
    pub delegator: String,
    #[serde(rename = "signature")]
    pub signature: String,
    #[serde(rename = "label")]
    pub label: String,
}

impl Delegate {
    pub fn new(delegate: String, delegator: String, signature: String, label: String) -> Delegate {
        Delegate {
            safe: None,
            delegate,
            delegator,
            signature,
            label,
        }
    }
}


