use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SafeBrowsingRequest {
    pub client: ClientInfo,
    pub threat_info: ThreatInfo,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    pub client_id: String,
    pub client_version: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThreatInfo {
    pub threat_types: Vec<ThreatType>,
    pub platform_types: Vec<String>,
    pub threat_entry_types: Vec<String>,
    pub threat_entries: Vec<ThreatEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ThreatEntry {
    pub url: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ThreatType {
    Malware,
    SocialEngineering,
    UnwantedSoftware,
    #[allow(dead_code)]
    PotentiallyHarmfulApplication,
}

#[derive(Deserialize, Debug)]
pub struct SafeBrowsingResponse {
    pub matches: Option<Vec<ThreatMatch>>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThreatMatch {
    pub threat_type: String,
    pub platform_type: String,
    pub threat: ThreatEntry,
}
