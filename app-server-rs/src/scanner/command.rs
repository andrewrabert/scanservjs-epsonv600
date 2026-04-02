use crate::scanner::ScanParams;

pub fn build_scan_command(base_cmd: &str, params: &ScanParams) -> Vec<String> {
    let mut args: Vec<String> = vec![base_cmd.to_string()];

    args.extend(["-d".to_string(), params.device_id.clone()]);

    if let Some(source) = &params.source {
        args.extend(["--source".to_string(), source.clone()]);
    }
    if let Some(mode) = &params.mode {
        args.extend(["--mode".to_string(), mode.clone()]);
    }
    if let Some(gamma) = &params.gamma_correction {
        args.extend(["--gamma-correction".to_string(), gamma.clone()]);
    }
    if let Some(cc) = &params.color_correction {
        args.extend(["--color-correction".to_string(), cc.clone()]);
    }
    if let Some(cs) = &params.color_space {
        args.extend(["--color-space".to_string(), cs.clone()]);
    }

    args.extend(["--resolution".to_string(), params.resolution.to_string()]);

    if let Some(left) = params.left {
        args.extend(["-l".to_string(), left.to_string()]);
    }
    if let Some(top) = params.top {
        args.extend(["-t".to_string(), top.to_string()]);
    }
    if let Some(width) = params.width {
        args.extend(["-x".to_string(), width.to_string()]);
    }
    if let Some(height) = params.height {
        args.extend(["-y".to_string(), height.to_string()]);
    }

    if let Some(format) = &params.format {
        args.extend(["--format".to_string(), format.clone()]);
    }

    if let Some(depth) = &params.depth {
        args.extend(["--depth".to_string(), depth.clone()]);
    }
    if let Some(brightness) = params.brightness {
        args.extend(["--brightness".to_string(), brightness.to_string()]);
    }
    if let Some(contrast) = params.contrast {
        if params.mode.as_deref() != Some("Lineart") {
            args.extend(["--contrast".to_string(), contrast.to_string()]);
        }
    }
    if params.mode.as_deref() == Some("Lineart") {
        if params.dynamic_lineart == Some(false) {
            args.push("--disable-dynamic-lineart=yes".to_string());
        }
    }

    if params.is_preview == Some(true) {
        args.push("--preview".to_string());
    } else {
        args.extend(["-o".to_string(), "output".to_string()]);
    }

    args
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_params() -> ScanParams {
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
    fn basic_scan_command() {
        let params = base_params();
        let args = build_scan_command("/usr/bin/scanservjs-epsonv600-scanimage", &params);

        assert_eq!(args[0], "/usr/bin/scanservjs-epsonv600-scanimage");
        assert_eq!(args[1], "-d");
        assert_eq!(args[2], "EpsonPerfectionV600");
        assert!(args.contains(&"--resolution".to_string()));
        assert!(args.contains(&"400".to_string()));
        assert!(args.contains(&"--format".to_string()));
        assert!(args.contains(&"tiff".to_string()));
        assert!(args.contains(&"-o".to_string()));
    }

    #[test]
    fn preview_uses_preview_flag() {
        let mut params = base_params();
        params.is_preview = Some(true);
        let args = build_scan_command("scanimage", &params);

        assert!(args.contains(&"--preview".to_string()));
        assert!(!args.contains(&"-o".to_string()));
    }

    #[test]
    fn lineart_mode_disables_contrast() {
        let mut params = base_params();
        params.mode = Some("Lineart".to_string());
        params.contrast = Some(50);
        params.dynamic_lineart = Some(false);
        let args = build_scan_command("scanimage", &params);

        assert!(!args.contains(&"--contrast".to_string()));
        assert!(args.contains(&"--disable-dynamic-lineart=yes".to_string()));
    }

    #[test]
    fn omits_optional_params_when_none() {
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
        let args = build_scan_command("scanimage", &params);

        assert!(!args.contains(&"--mode".to_string()));
        assert!(!args.contains(&"--source".to_string()));
        assert!(!args.contains(&"--brightness".to_string()));
        assert!(args.contains(&"-d".to_string()));
        assert!(args.contains(&"--resolution".to_string()));
        assert!(args.contains(&"-o".to_string()));
    }
}
