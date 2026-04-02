pub mod command;
pub mod request;

use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScanRequest {
    pub params: ScanParams,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScanParams {
    pub device_id: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub resolution: u32,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub top: Option<f64>,
    #[serde(default)]
    pub left: Option<f64>,
    #[serde(default)]
    pub width: Option<f64>,
    #[serde(default)]
    pub height: Option<f64>,
    #[serde(default)]
    pub brightness: Option<i32>,
    #[serde(default)]
    pub contrast: Option<i32>,
    #[serde(default)]
    pub gamma_correction: Option<String>,
    #[serde(default)]
    pub color_correction: Option<String>,
    #[serde(default)]
    pub color_space: Option<String>,
    #[serde(default)]
    pub depth: Option<String>,
    #[serde(default)]
    pub dynamic_lineart: Option<bool>,
    #[serde(default)]
    pub is_preview: Option<bool>,
}

fn deserialize_number_from_string<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de;
    struct NumberVisitor;
    impl<'de> de::Visitor<'de> for NumberVisitor {
        type Value = u32;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a number or string containing a number")
        }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<u32, E> {
            u32::try_from(v).map_err(E::custom)
        }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<u32, E> {
            v.parse().map_err(E::custom)
        }
    }
    deserializer.deserialize_any(NumberVisitor)
}
