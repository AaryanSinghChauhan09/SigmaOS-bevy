//! Configuration implementation with security defaults

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use log::{debug, warn};

/// Runtime configuration with secure defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub config_dir: String,
    pub data_dir: String,
    pub cache_dir: String,
    pub log_dir: String,
    pub log_level: String,
    pub enable_tracing: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            config_dir: "/etc/sigmaos".to_string(),
            data_dir: "/var/lib/sigmaos".to_string(),
            cache_dir: "/var/cache/sigmaos".to_string(),
            log_dir: "/var/log/sigmaos".to_string(),
            log_level: "info".to_string(),
            enable_tracing: true,
        }
    }
}

impl RuntimeConfig {
    /// Load default configuration with security defaults
    pub fn load_default() -> Result<Self> {
        debug!("Loading default configuration with secure defaults");
        Ok(Self::default())
    }

    /// Load from file with validation
    pub fn load_from_file(path: &str) -> Result<Self> {
        debug!("Loading configuration from: {}", path);
        
        if Path::new(path).exists() {
            let content = std::fs::read_to_string(path)?;
            let config: RuntimeConfig = serde_json::from_str(&content)?;
            
            config.validate()?;
            
            debug!("Configuration loaded and validated successfully");
            Ok(config)
        } else {
            warn!("Configuration file not found, using defaults");
            Ok(Self::default())
        }
    }

    /// Validate configuration for security
    pub fn validate(&self) -> Result<()> {
        debug!("Validating configuration");
        
        match self.log_level.as_str() {
            "trace" | "debug" | "info" | "warn" | "error" => {},
            _ => {
                warn!("Invalid log level, defaulting to info");
                return Err(anyhow::anyhow!("Invalid log level"));
            }
        }
        
        debug!("Configuration validation passed");
        Ok(())
    }

    /// Save to file with secure permissions
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        debug!("Saving configuration to: {}", path);
        
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(path)?.permissions();
            perms.set_mode(0o600);
            std::fs::set_permissions(path, perms)?;
        }
        
        debug!("Configuration saved with secure permissions");
        Ok(())
    }
}
