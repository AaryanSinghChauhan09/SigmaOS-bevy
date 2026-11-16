//! runtime module

use anyhow::Result;
use log::debug;

pub struct RuntimeManager;

impl RuntimeManager {
    pub fn new(_config: crate::config::RuntimeConfig) -> Result<Self> {
        debug!("Creating Runtime manager");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing Runtime manager");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down Runtime manager");
        Ok(())
    }
}
