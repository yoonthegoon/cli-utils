use cli_utils::error::Error;
use cli_utils::result::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
struct Config {
    aliases_path: PathBuf,
    rc_path: PathBuf,
}

impl Config {
    fn new(path: PathBuf) -> Result<Config> {
        let aliases_path = Path::new(env!("HOME")).join(".aliases.sh");
        let rc_path = Path::new(env!("HOME")).join(".zshrc"); // TODO: find rc file
        let config = Config {
            aliases_path,
            rc_path,
        };
        let config_string = toml::to_string(&config).unwrap();
        fs::write(path, config_string)?;
        Ok(config)
    }

    fn from(path: PathBuf) -> Result<Config> {
        let config_string = fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&config_string).unwrap_or(Config::new(path)?);
        Ok(config)
    }
}

struct Alias {
    name: String,
    string: String,
}

impl Alias {
    fn new(name: String, string: String) -> Alias {
        Alias { name, string }
    }
}

impl Into<String> for Alias {
    fn into(self) -> String {
        format!("alias {}=\"{}\"", self.name, self.string)
    }
}

impl TryFrom<String> for Alias {
    type Error = Error;

    fn try_from(value: String) -> Result<Alias> {
        // https://github.com/bminor/bash/blob/ec8113b9861375e4e17b3307372569d429dec814/general.c#L412-L426
        let re = Regex::new(r#"^alias (?<name>[^\s/\\$`=|&;()<>'"])="(?<string>.*)"$"#).unwrap();
        let caps = re.captures(&value).ok_or(Error::OptionNone)?;
        let name = caps
            .name("name")
            .ok_or(Error::OptionNone)?
            .as_str()
            .to_owned();
        let string = caps
            .name("string")
            .ok_or(Error::OptionNone)?
            .as_str()
            .to_owned();
        Ok(Alias::new(name, string))
    }
}

fn add_alias(name: &str, string: &str, aliases_file: &File) -> Result<()> {
    let aliases = get_aliases(aliases_file)?;
    todo!()
}

fn edit_alias(name: &str, string: &str, aliases_file: &File) -> Result<()> {
    let aliases = get_aliases(aliases_file)?;
    todo!()
}

fn get_aliases(aliases_file: &File) -> Result<Vec<Alias>> {
    todo!()
}

fn remove_alias(name: &str, aliases_file: &File) -> Result<()> {
    let aliases = get_aliases(aliases_file)?;
    todo!()
}

fn get_config() -> Result<Config> {
    let path = Path::new(env!("HOME")).join(".config/alias-manager/config.toml");
    Config::from(path)
}

// commands

fn add(name: &str, string: &str) -> Result<()> {
    let config = get_config()?;
    let alias_file = File::open(config.aliases_path)?;
    add_alias(name, string, &alias_file)
}

fn edit(name: &str, string: &str) -> Result<()> {
    let config = get_config()?;
    let alias_file = File::open(config.aliases_path)?;
    edit_alias(name, string, &alias_file)
}

fn list() -> Result<Vec<Alias>> {
    let config = get_config()?;
    let alias_file = File::open(config.aliases_path)?;
    get_aliases(&alias_file)
}

fn remove(name: &str) -> Result<()> {
    let config = get_config()?;
    let alias_file = File::open(config.aliases_path)?;
    remove_alias(name, &alias_file)
}

fn main() {}
