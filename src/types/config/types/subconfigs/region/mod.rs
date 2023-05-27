use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LatLong {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub endpoint: String,
    pub location: Option<LatLong>,
    pub vip: bool,
    pub custom: bool,
    #[serde(default)]
    pub deprecated: bool,
}
