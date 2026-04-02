use serde::Deserialize;

fn default_timeout() -> u64 { 600 }
fn default_preview_resolution() -> u32 { 200 }
fn default_log_level() -> String { "info".to_string() }
fn default_thumbnail_dir() -> String { "/data/thumbnails".to_string() }
fn default_preview_dir() -> String { "/data/preview".to_string() }
fn default_client_dir() -> String { "./client".to_string() }
fn default_log() -> Log {
    Log { level: default_log_level() }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub paths: Paths,
    pub scanner: Scanner,
    #[serde(default = "default_log")]
    pub log: Log,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

#[derive(Debug, Deserialize)]
pub struct Paths {
    pub output_dir: String,
    #[serde(default = "default_thumbnail_dir")]
    pub thumbnail_dir: String,
    #[serde(default = "default_preview_dir")]
    pub preview_dir: String,
    #[serde(default = "default_client_dir")]
    pub client_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct Scanner {
    pub command: String,
    #[serde(default = "default_preview_resolution")]
    pub preview_resolution: u32,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    #[serde(default = "default_log_level")]
    pub level: String,
}

impl Config {
    pub fn load(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full_config() {
        let toml = r#"
[server]
host = "::"
port = 8080
timeout_secs = 600

[paths]
output_dir = "/data/scans"
thumbnail_dir = "/data/thumbnails"
preview_dir = "/data/preview"
client_dir = "./client"

[scanner]
command = "/usr/bin/scanservjs-epsonv600-scanimage"
preview_resolution = 200

[log]
level = "info"
"#;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.server.host, "::");
        assert_eq!(config.server.timeout_secs, 600);
        assert_eq!(config.paths.output_dir, "/data/scans");
        assert_eq!(config.paths.thumbnail_dir, "/data/thumbnails");
        assert_eq!(config.paths.preview_dir, "/data/preview");
        assert_eq!(config.paths.client_dir, "./client");
        assert_eq!(config.scanner.command, "/usr/bin/scanservjs-epsonv600-scanimage");
        assert_eq!(config.scanner.preview_resolution, 200);
        assert_eq!(config.log.level, "info");
    }

    #[test]
    fn defaults_for_optional_fields() {
        let toml = r#"
[server]
host = "0.0.0.0"
port = 9090

[paths]
output_dir = "/tmp/scans"

[scanner]
command = "scanimage"
"#;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.server.port, 9090);
        assert_eq!(config.server.timeout_secs, 600);
        assert_eq!(config.scanner.preview_resolution, 200);
        assert_eq!(config.log.level, "info");
    }
}
