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
        fs::write(path, toml::to_string(&config))?; // FIXME: error[E0277]: the trait bound `Result<std::string::String, toml::ser::Error>: AsRef<[u8]>` is not satisfied
        Ok(config)
    }

    fn from(path: PathBuf) -> Result<Config> {
        let file = File::open(&path)?.into();
        let config: Config = toml::from_str(file).unwrap_or(Config::new(path)?);
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

fn add_command(name: &str, string: &str) -> Result<String> {
    todo!()
}

fn edit_command(name: &str, string: &str) -> Result<()> {
    todo!()
}

fn list_command() -> Result<Vec<Alias>> {
    todo!()
}

fn remove_command(name: &str) -> Result<()> {
    todo!()
}

fn add_helper(name: &str, string: &str, aliases_file: File) -> Result<String> {
    todo!()
}

fn edit_helper(name: &str, string: &str, aliases_file: File) -> Result<()> {
    todo!()
}

fn list_helper(aliases_file: File) -> Result<Vec<Alias>> {
    todo!()
}

fn remove_helper(name: &str, aliases_file: File) -> Result<()> {
    todo!()
}

fn get_aliases_file() -> Result<File> {
    let conf_path = Path::new(env!("HOME")).join(".config/am/config.toml");
    if !conf_path.exists() {
        return Err(Error::ConfigDoesNotExist);
    }
    todo!()
}

fn main() {}
