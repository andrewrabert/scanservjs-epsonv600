use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub features: std::collections::HashMap<String, Feature>,
    pub settings: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Feature {
    pub text: String,
    pub name: String,
    pub default: serde_json::Value,
    pub parameters: String,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<[f64; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<f64>,
}

fn feature_enum(name: &str, text: &str, params: &str, options: &[&str], default: &str) -> Feature {
    Feature {
        text: text.to_string(),
        name: name.to_string(),
        default: serde_json::json!(default),
        parameters: params.to_string(),
        enabled: true,
        meta: None,
        options: Some(options.iter().map(|o| serde_json::json!(o)).collect()),
        limits: None,
        interval: None,
    }
}

fn feature_geometry(name: &str, text: &str, min: f64, max: f64, default: f64) -> Feature {
    Feature {
        text: text.to_string(),
        name: name.to_string(),
        default: serde_json::json!(default),
        parameters: format!("{}..{}mm", min, max),
        enabled: true,
        meta: None,
        options: None,
        limits: Some([min, max]),
        interval: Some(1.0),
    }
}

fn feature_resolution(name: &str, text: &str, options: &[u32], default: u32) -> Feature {
    let min = *options.first().unwrap() as f64;
    let max = *options.last().unwrap() as f64;
    Feature {
        text: text.to_string(),
        name: name.to_string(),
        default: serde_json::json!(default),
        parameters: options.iter().map(|o| o.to_string()).collect::<Vec<_>>().join("|"),
        enabled: true,
        meta: None,
        options: Some(options.iter().map(|o| serde_json::json!(o)).collect()),
        limits: Some([min, max]),
        interval: Some(1.0),
    }
}

fn feature_range(name: &str, text: &str, min: f64, max: f64, step: f64, default: f64) -> Feature {
    Feature {
        text: text.to_string(),
        name: name.to_string(),
        default: serde_json::json!(default),
        parameters: format!("{}..{}", min, max),
        enabled: true,
        meta: None,
        options: None,
        limits: Some([min, max]),
        interval: Some(step),
    }
}

pub fn epson_v600() -> Device {
    let mut features = std::collections::HashMap::new();

    features.insert("--mode".to_string(), feature_enum(
        "--mode",
        "--mode Binary|Gray|Color [Color]",
        "Binary|Gray|Color",
        &["Binary", "Gray", "Color"],
        "Color",
    ));

    features.insert("--depth".to_string(), feature_enum(
        "--depth",
        "--depth 8|16 [8]",
        "8|16",
        &["8", "16"],
        "8",
    ));

    features.insert("--resolution".to_string(), feature_resolution(
        "--resolution",
        "--resolution 200|400|800|1600|3200|6400dpi [400]",
        &[200, 400, 800, 1600, 3200, 6400],
        400,
    ));

    features.insert("-l".to_string(), feature_geometry(
        "-l", "-l 0..215.9mm [0]", 0.0, 215.9, 0.0,
    ));
    features.insert("-t".to_string(), feature_geometry(
        "-t", "-t 0..297.18mm [0]", 0.0, 297.18, 0.0,
    ));
    features.insert("-x".to_string(), feature_geometry(
        "-x", "-x 0..215.9mm [215.9]", 0.0, 215.9, 215.9,
    ));
    features.insert("-y".to_string(), feature_geometry(
        "-y", "-y 0..297.18mm [297.18]", 0.0, 297.18, 297.18,
    ));

    features.insert("--format".to_string(), feature_enum(
        "--format",
        "--format jpeg|jpeg-xl|png|ppm|tiff [tiff]",
        "jpeg|jpeg-xl|png|ppm|tiff",
        &["jpeg", "jpeg-xl", "png", "ppm", "tiff"],
        "tiff",
    ));

    features.insert("--source".to_string(), feature_enum(
        "--source",
        "--source Flatbed|Transparency Unit [Flatbed]",
        "Flatbed|Transparency Unit",
        &["Flatbed", "Transparency Unit"],
        "Flatbed",
    ));

    features.insert("--gamma-correction".to_string(), feature_enum(
        "--gamma-correction",
        "--gamma-correction 1.0|1.8|2.0|2.2|2.4|2.8 [2.2]",
        "1.0|1.8|2.0|2.2|2.4|2.8",
        &["1.0", "1.8", "2.0", "2.2", "2.4", "2.8"],
        "2.2",
    ));

    features.insert("--color-correction".to_string(), feature_enum(
        "--color-correction",
        "--color-correction[=(yes|no)] [yes]",
        "yes|no",
        &["yes", "no"],
        "yes",
    ));

    features.insert("--color-space".to_string(), feature_enum(
        "--color-space",
        "--color-space None|Adobe RGB (1998)|EPSON sRGB|sRGB IEC61966-2.1 [sRGB IEC61966-2.1]",
        "None|Adobe RGB (1998)|EPSON sRGB|sRGB IEC61966-2.1",
        &["None", "Adobe RGB (1998)", "EPSON sRGB", "sRGB IEC61966-2.1"],
        "sRGB IEC61966-2.1",
    ));

    features.insert("--brightness".to_string(), feature_range(
        "--brightness",
        "--brightness -100..100% (in steps of 1) [0]",
        -100.0, 100.0, 1.0, 0.0,
    ));

    features.insert("--contrast".to_string(), feature_range(
        "--contrast",
        "--contrast -100..100% (in steps of 1) [0]",
        -100.0, 100.0, 1.0, 0.0,
    ));

    features.insert("--preview".to_string(), feature_enum(
        "--preview",
        "--preview yes|no [no]",
        "yes|no",
        &["yes", "no"],
        "no",
    ));

    features.insert("--speed".to_string(), feature_enum(
        "--speed",
        "--speed yes|no [no]",
        "yes|no",
        &["yes", "no"],
        "no",
    ));

    features.insert("--scan-area".to_string(), feature_enum(
        "--scan-area",
        "--scan-area Maximum|A4|A5 Landscape|A5 Portrait|B5|Letter|Executive|CD [Maximum]",
        "Maximum|A4|A5 Landscape|A5 Portrait|B5|Letter|Executive|CD",
        &["Maximum", "A4", "A5 Landscape", "A5 Portrait", "B5", "Letter", "Executive", "CD"],
        "Maximum",
    ));

    Device {
        id: "EpsonPerfectionV600".to_string(),
        name: "EpsonPerfectionV600".to_string(),
        features,
        settings: serde_json::json!({}),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epson_v600_has_expected_features() {
        let device = epson_v600();
        assert_eq!(device.id, "EpsonPerfectionV600");

        let res = &device.features["--resolution"];
        assert_eq!(res.default, serde_json::json!(400));
        let opts: Vec<serde_json::Value> = vec![200, 400, 800, 1600, 3200, 6400]
            .into_iter()
            .map(|v| serde_json::json!(v))
            .collect();
        assert_eq!(res.options.as_ref().unwrap(), &opts);
        assert_eq!(res.limits, Some([200.0, 6400.0]));

        let mode = &device.features["--mode"];
        assert_eq!(mode.default, serde_json::json!("Color"));
        let mode_opts: Vec<serde_json::Value> = ["Binary", "Gray", "Color"]
            .iter()
            .map(|v| serde_json::json!(v))
            .collect();
        assert_eq!(mode.options.as_ref().unwrap(), &mode_opts);

        let source = &device.features["--source"];
        assert_eq!(source.default, serde_json::json!("Flatbed"));

        let x = &device.features["-x"];
        assert_eq!(x.limits, Some([0.0, 215.9]));
        assert_eq!(x.default, serde_json::json!(215.9));

        let gamma = &device.features["--gamma-correction"];
        assert_eq!(gamma.default, serde_json::json!("2.2"));

        let color_space = &device.features["--color-space"];
        assert_eq!(color_space.default, serde_json::json!("sRGB IEC61966-2.1"));

        let format = &device.features["--format"];
        assert_eq!(format.default, serde_json::json!("tiff"));
    }

    #[test]
    fn epson_v600_serializes_to_json() {
        let device = epson_v600();
        let json = serde_json::to_value(&device).unwrap();
        assert!(json["features"]["--resolution"]["options"].is_array());
        assert!(json["features"]["--mode"]["options"].is_array());
        assert!(json["settings"].is_object());
    }
}
