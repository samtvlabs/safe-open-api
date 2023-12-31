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
pub struct FirebaseDeviceSerializerWithOwnersResponse {
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<uuid::Uuid>,
    #[serde(rename = "safes")]
    pub safes: Vec<String>,
    #[serde(rename = "cloudMessagingToken")]
    pub cloud_messaging_token: String,
    #[serde(rename = "buildNumber")]
    pub build_number: i32,
    #[serde(rename = "bundle")]
    pub bundle: String,
    #[serde(rename = "deviceType")]
    pub device_type: DeviceType,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i32>,
    #[serde(rename = "signatures", skip_serializing_if = "Option::is_none")]
    pub signatures: Option<Vec<String>>,
    #[serde(rename = "ownersRegistered")]
    pub owners_registered: Vec<String>,
    #[serde(rename = "ownersNotRegistered")]
    pub owners_not_registered: Vec<String>,
}

impl FirebaseDeviceSerializerWithOwnersResponse {
    pub fn new(safes: Vec<String>, cloud_messaging_token: String, build_number: i32, bundle: String, device_type: DeviceType, version: String, owners_registered: Vec<String>, owners_not_registered: Vec<String>) -> FirebaseDeviceSerializerWithOwnersResponse {
        FirebaseDeviceSerializerWithOwnersResponse {
            uuid: None,
            safes,
            cloud_messaging_token,
            build_number,
            bundle,
            device_type,
            version,
            timestamp: None,
            signatures: None,
            owners_registered,
            owners_not_registered,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeviceType {
    #[serde(rename = "ANDROID")]
    Android,
    #[serde(rename = "IOS")]
    Ios,
    #[serde(rename = "WEB")]
    Web,
}

impl Default for DeviceType {
    fn default() -> DeviceType {
        Self::Android
    }
}

