use std::fs;

use anyhow::Result;
use ratatui::prelude::*;

use clap::crate_name;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Ui {
    #[serde_as(as = "DisplayFromStr")]
    pub hotkey_fg: Color,
    #[serde_as(as = "DisplayFromStr")]
    pub hotkey_bg: Color,
    #[serde_as(as = "DisplayFromStr")]
    pub selected_fg: Color,
    #[serde_as(as = "DisplayFromStr")]
    pub selected_bg: Color,
}

#[serde_as]
#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Highlight {
    #[serde_as(as = "DisplayFromStr")]
    pub base00: Color,
    #[serde_as(as = "DisplayFromStr")]
    pub base03: Color,
    #[serde_as(as = "DisplayFromStr")]
    pub base05: Color,
    #[serde_as(as = "DisplayFromStr")]
    pub base08: Color,
    #[serde_as(as = "DisplayFromStr")]
    pub base09: Color,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "base0A")]
    pub base0a: Color,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "base0B")]
    pub base0b: Color,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "base0C")]
    pub base0c: Color,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "base0D")]
    pub base0d: Color,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "base0E")]
    pub base0e: Color,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "base0F")]
    pub base0f: Color,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Config {
    pub ui: Ui,
    pub highlight: Highlight,
}

pub fn load_config() -> Result<Config> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(crate_name!())?;
    let config_file_name = format!("{}-config.toml", crate_name!());

    if let Some(filename) = xdg_dirs.find_config_file(&config_file_name) {
        Ok(toml::from_str(&fs::read_to_string(filename)?)?)
    } else {
        let default_config_str = include_str!("../config/config.toml");
        let default_config: Config = toml::from_str(default_config_str)?;

        if let Ok(filename) = xdg_dirs.place_config_file(&config_file_name) {
            fs::write(filename, default_config_str)?;
        }

        Ok(default_config)
    }
}
