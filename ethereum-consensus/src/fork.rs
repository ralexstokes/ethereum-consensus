// Identifies the fork of the protocol the associated object belongs to.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Fork {
    Phase0,
    Altair,
    Bellatrix,
    Capella,
    Deneb,
}
