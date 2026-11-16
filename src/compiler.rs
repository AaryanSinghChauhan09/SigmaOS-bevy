//! compiler module

use anyhow::Result;
use log::debug;

pub struct CompilerManager;

impl CompilerManager {
    pub fn new(_config: crate::config::RuntimeConfig) -> Result<Self> {
        debug!("Creating Compiler manager");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing Compiler manager");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down Compiler manager");
        Ok(())
    }
}
