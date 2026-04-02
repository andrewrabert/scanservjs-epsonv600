use crate::device::Device;
use crate::error::AppError;
use crate::scanner::ScanParams;

pub fn validate_params(params: &ScanParams, device: &Device) -> Result<(), AppError> {
    if params.device_id != device.id {
        return Err(AppError::BadRequest(format!(
            "Device {} not found",
            params.device_id
        )));
    }

    validate_option("--resolution", &Some(params.resolution.to_string()), device)?;

    if let Some(mode) = &params.mode {
        validate_option("--mode", &Some(mode.clone()), device)?;
    }
    if let Some(format) = &params.format {
        validate_option("--format", &Some(format.clone()), device)?;
    }
    if let Some(source) = &params.source {
        validate_option("--source", &Some(source.clone()), device)?;
    }
    if let Some(gamma) = &params.gamma_correction {
        validate_option("--gamma-correction", &Some(gamma.clone()), device)?;
    }
    if let Some(cc) = &params.color_correction {
        validate_option("--color-correction", &Some(cc.clone()), device)?;
    }
    if let Some(cs) = &params.color_space {
        validate_option("--color-space", &Some(cs.clone()), device)?;
    }
    if let Some(depth) = &params.depth {
        validate_option("--depth", &Some(depth.clone()), device)?;
    }

    if let Some(left) = params.left {
        validate_range("-l", left, device)?;
    }
    if let Some(top) = params.top {
        validate_range("-t", top, device)?;
    }
    if let Some(width) = params.width {
        validate_range("-x", width, device)?;
    }
    if let Some(height) = params.height {
        validate_range("-y", height, device)?;
    }

    Ok(())
}

fn validate_option(
    feature_name: &str,
    value: &Option<String>,
    device: &Device,
) -> Result<(), AppError> {
    let Some(value) = value else { return Ok(()) };
    let Some(feature) = device.features.get(feature_name) else {
        return Err(AppError::BadRequest(format!(
            "{} is missing from device",
            feature_name,
        )));
    };
    if let Some(options) = &feature.options {
        let valid = options.iter().any(|o| {
            o.as_str().map(|s| s == value).unwrap_or(false)
                || o.as_u64().map(|n| n.to_string() == *value).unwrap_or(false)
                || o.as_f64().map(|n| n.to_string() == *value).unwrap_or(false)
        });
        if !valid {
            return Err(AppError::BadRequest(format!(
                "Invalid {}: '{}' not in '{:?}'",
                feature_name, value, options
            )));
        }
    }
    Ok(())
}

fn validate_range(feature_name: &str, value: f64, device: &Device) -> Result<(), AppError> {
    let Some(feature) = device.features.get(feature_name) else {
        return Err(AppError::BadRequest(format!(
            "{} is missing from device",
            feature_name,
        )));
    };
    if let Some(limits) = &feature.limits {
        if value < limits[0] || value > limits[1] {
            return Err(AppError::BadRequest(format!(
                "{}: {} out of range [{}, {}]",
                feature_name, value, limits[0], limits[1]
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::epson_v600;

    fn valid_params() -> ScanParams {
        ScanParams {
            device_id: "EpsonPerfectionV600".to_string(),
            resolution: 400,
            mode: Some("Color".to_string()),
            format: Some("tiff".to_string()),
            source: Some("Flatbed".to_string()),
            top: Some(0.0),
            left: Some(0.0),
            width: Some(215.9),
            height: Some(297.18),
            brightness: None,
            contrast: None,
            gamma_correction: Some("2.2".to_string()),
            color_correction: Some("yes".to_string()),
            color_space: Some("sRGB IEC61966-2.1".to_string()),
            depth: Some("8".to_string()),
            dynamic_lineart: None,
            is_preview: None,
        }
    }

    #[test]
    fn valid_params_pass() {
        let device = epson_v600();
        assert!(validate_params(&valid_params(), &device).is_ok());
    }

    #[test]
    fn wrong_device_id_fails() {
        let device = epson_v600();
        let mut params = valid_params();
        params.device_id = "WrongDevice".to_string();
        assert!(validate_params(&params, &device).is_err());
    }

    #[test]
    fn invalid_resolution_fails() {
        let device = epson_v600();
        let mut params = valid_params();
        params.resolution = 999;
        assert!(validate_params(&params, &device).is_err());
    }

    #[test]
    fn invalid_mode_fails() {
        let device = epson_v600();
        let mut params = valid_params();
        params.mode = Some("HDR".to_string());
        assert!(validate_params(&params, &device).is_err());
    }

    #[test]
    fn geometry_out_of_range_fails() {
        let device = epson_v600();
        let mut params = valid_params();
        params.width = Some(300.0);
        assert!(validate_params(&params, &device).is_err());
    }

    #[test]
    fn none_optionals_pass() {
        let device = epson_v600();
        let params = ScanParams {
            device_id: "EpsonPerfectionV600".to_string(),
            resolution: 200,
            mode: None,
            format: None,
            source: None,
            top: None,
            left: None,
            width: None,
            height: None,
            brightness: None,
            contrast: None,
            gamma_correction: None,
            color_correction: None,
            color_space: None,
            depth: None,
            dynamic_lineart: None,
            is_preview: None,
        };
        assert!(validate_params(&params, &device).is_ok());
    }
}
