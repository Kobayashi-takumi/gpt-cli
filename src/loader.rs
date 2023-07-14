use anyhow::{anyhow, Result};
use etcetera::{choose_base_strategy, BaseStrategy};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub static CONFIG_FILE: once_cell::sync::OnceCell<PathBuf> = once_cell::sync::OnceCell::new();
pub static LOG_FILE: once_cell::sync::OnceCell<PathBuf> = once_cell::sync::OnceCell::new();

fn ensure_parent_dir(path: &Path) {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).ok();
        }
    }
}

fn config_dir() -> Result<PathBuf> {
    let strategy = choose_base_strategy().expect("Unable to find the config directory!");
    let mut path = strategy.config_dir();
    path.push("gpt-cli");
    let path = path.join("config.toml");
    Ok(path)
}

fn log_dir() -> Result<PathBuf> {
    let strategy = choose_base_strategy().expect("Unable to find the config directory!");
    let mut path = strategy.cache_dir();
    path.push("gpt-cli");
    let path = path.join("gpt-cli.log");
    Ok(path)
}

pub fn config_file() -> Result<PathBuf> {
    match CONFIG_FILE.get().map(|path| path.to_path_buf()) {
        Some(val) => Ok(val),
        None => Err(anyhow!("Failed to load config file.")),
    }
}

pub fn log_file() -> Result<PathBuf> {
    match LOG_FILE.get().map(|path| path.to_path_buf()) {
        Some(val) => Ok(val),
        None => Err(anyhow!("Failed to load log file.")),
    }
}

pub fn initialize_config_file() -> Result<()> {
    let config_file = config_dir()?;
    ensure_parent_dir(&config_file);
    match CONFIG_FILE.set(config_file) {
        Ok(_) => {}
        Err(_) => return Err(anyhow!("Failed to set a config file.")),
    };
    Ok(())
}

pub fn initialize_log_file() -> Result<()> {
    let log_file = log_dir()?;
    ensure_parent_dir(&log_file);
    match LOG_FILE.set(log_file) {
        Ok(_) => {}
        Err(_) => return Err(anyhow!("Failed to set a log file.")),
    };
    Ok(())
}
