mod _impl;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SensorConfig {
    pub p1: PinConfig,
    pub p2: PinConfig,
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct PinConfig {
    pub s1: PortMode,
    pub s2: PortMode,
}

#[derive(Clone, Default, PartialEq, Serialize, Debug)]
#[serde(rename_all = "lowercase", tag = "mode", content = "config")]
pub enum PortMode {
    #[default]
    Inactive,
    Sio(SioConfig),
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct SioConfig {
    pub mode: SioMode,
    pub debounce_us: u32,
}

#[derive(Clone, Copy, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SioMode {
    Npn,
    #[default]
    Pnp,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_serialized_pin_config() {
        let data = SensorConfig::default();

        let serialized = serde_json_core::to_string::<_, 512>(&data).unwrap();

        let (deserialized, _) = serde_json_core::from_str::<SensorConfig>(&serialized).unwrap();

        assert_eq!(deserialized, data);
    }

    #[test]
    fn persistance_roundtrip_postcard() {
        const MAX_PAYLOAD_SIZE: usize = 512;

        let data = SensorConfig::default();

        let buf = &mut [0u8; MAX_PAYLOAD_SIZE];

        let serialized = postcard::to_slice(&data, buf).unwrap();

        let deserialized = postcard::from_bytes::<SensorConfig>(serialized).unwrap();

        assert_eq!(deserialized, data);
    }
}
