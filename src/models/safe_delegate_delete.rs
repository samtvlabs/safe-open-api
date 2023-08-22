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
pub struct SafeDelegateDelete {
    #[serde(rename = "safe")]
    pub safe: String,
    #[serde(rename = "delegate")]
    pub delegate: String,
    #[serde(rename = "signature")]
    pub signature: String,
}

impl SafeDelegateDelete {
    pub fn new(safe: String, delegate: String, signature: String) -> SafeDelegateDelete {
        SafeDelegateDelete {
            safe,
            delegate,
            signature,
        }
    }
}


