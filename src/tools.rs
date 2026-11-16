//! tools module

use anyhow::Result;
use log::debug;

pub struct ToolsManager;

impl ToolsManager {
    pub fn new(_config: crate::config::RuntimeConfig) -> Result<Self> {
        debug!("Creating Tools manager");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing Tools manager");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down Tools manager");
        Ok(())
    }
}
