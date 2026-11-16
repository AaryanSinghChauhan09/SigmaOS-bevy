//! package module

use anyhow::Result;
use log::debug;

pub struct PackageManager;

impl PackageManager {
    pub fn new(_config: crate::config::RuntimeConfig) -> Result<Self> {
        debug!("Creating Package manager");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing Package manager");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down Package manager");
        Ok(())
    }
}
