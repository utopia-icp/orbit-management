use candid::{CandidType, Deserialize};

pub type TimestampRfc3339 = String;

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CanisterInit {
    pub upgrader_wasm_module: Vec<u8>,
    pub station_wasm_module: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CanisterUpgrade {
    pub upgrader_wasm_module: Option<Vec<u8>>,
    pub station_wasm_module: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum CanisterInstall {
    Init(CanisterInit),
    Upgrade(CanisterUpgrade),
}