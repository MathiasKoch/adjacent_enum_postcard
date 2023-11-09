mod _impl;

use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct PinConfig {
    pub s1: PortMode,
    pub s2: PortMode,
}

#[derive(Default, PartialEq, Serialize, Debug)]
#[serde(rename_all = "lowercase", tag = "mode", content = "config")]
pub enum PortMode {
    #[default]
    Inactive,
    Sio(SioConfig),
}

#[derive(Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct SioConfig {
    pub mode: SioMode,
    pub debounce_us: u32,
}

#[derive(Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SioMode {
    Npn,
    #[default]
    Pnp,
}

#[cfg(test)]
mod tests {
    use super::*;
    const MAX_PAYLOAD_SIZE: usize = 512;

    #[test]
    fn roundtrip_json() {
        let data = PinConfig::default();
        let buf = &mut [0u8; MAX_PAYLOAD_SIZE];
        let len = serde_json_core::to_slice(&data, buf).unwrap();
        let (deserialized, _) = serde_json_core::from_slice::<PinConfig>(&buf[..len]).unwrap();
        assert_eq!(deserialized, data);
    }

    #[test]
    fn roundtrip_postcard() {
        let data = PinConfig::default();
        let buf = &mut [0u8; MAX_PAYLOAD_SIZE];
        let serialized = postcard::to_slice(&data, buf).unwrap();
        let deserialized = postcard::from_bytes::<PinConfig>(serialized).unwrap();
        assert_eq!(deserialized, data);
    }
}
