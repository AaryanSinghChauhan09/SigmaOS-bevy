//! SigmaOS Bevy Runtime Compatibility Layer
//!
//! Bevy runtime compatibility for SigmaOS
//! Provides runtime environment and execution

#![forbid(unsafe_code)]

use anyhow::Result;
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod runtime;
pub mod compiler;
pub mod package;
pub mod tools;
pub mod error;
pub mod config;

pub use runtime::*;
pub use compiler::*;
pub use package::*;
pub use tools::*;
pub use error::*;
pub use config::*;

/// Runtime compatibility layer
pub struct Runtime {
    pub config: RuntimeConfig,
    pub environment: Arc<RwLock<Environment>>,
    pub package_manager: Arc<RwLock<PackageManager>>,
}

impl Runtime {
    /// Create new runtime instance
    pub fn new() -> Result<Self> {
        info!("Initializing SigmaOS Bevy runtime");
        let config = RuntimeConfig::load_default()?;
        debug!("Loaded runtime configuration");
        
        let environment = Arc::new(RwLock::new(Environment::new(config.clone())?));
        let package_manager = Arc::new(RwLock::new(PackageManager::new(config.clone())?));
        
        Ok(Self {
            config,
            environment,
            package_manager,
        })
    }

    /// Initialize runtime
    pub async fn init(&mut self) -> Result<()> {
        info!("Starting runtime initialization");
        
        let mut environment = self.environment.write().await;
        environment.init().await?;
        debug!("Environment initialized");
        
        let mut package_manager = self.package_manager.write().await;
        package_manager.init().await?;
        debug!("Package manager initialized");
        
        info!("Runtime initialization complete");
        Ok(())
    }

    /// Shutdown runtime
    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down runtime");
        
        let mut environment = self.environment.write().await;
        environment.shutdown().await?;
        
        let mut package_manager = self.package_manager.write().await;
        package_manager.shutdown().await?;
        
        info!("Runtime shutdown complete");
        Ok(())
    }
}
