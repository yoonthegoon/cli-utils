use clap::{Args, Parser, Subcommand};
use cli_utils::error::Error;
use cli_utils::result::Result;
use prettytable::{format, row, Table};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

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
        let re = Regex::new(r#"^alias (?<name>[^\s/\\$`=|&;()<>'"]+)="(?<string>.*)"$"#).unwrap();
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

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a managed alias
    Add { name: String, string: String },
    /// Edit a managed alias
    Edit { name: String, string: String },
    /// List all managed aliases
    List,
    /// Remove a managed alias
    Remove { name: String },
}

#[derive(Serialize, Deserialize)]
struct Config {
    aliases_path: PathBuf,
    rc_path: PathBuf,
}

impl Config {
    fn new(path: PathBuf) -> Result<Config> {
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        let aliases_path = Path::new(env!("HOME")).join(".aliases.sh");
        let rc_path = Path::new(env!("HOME")).join(".zshrc"); // TODO: find rc file
        let config = Config {
            aliases_path,
            rc_path,
        };
        let config_string = toml::to_string(&config)?;
        fs::write(path, config_string)?;
        Ok(config)
    }

    fn from(path: PathBuf) -> Result<Config> {
        let config_string = fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&config_string).unwrap_or(Config::new(path)?);
        Ok(config)
    }
}

fn add_alias(name: &str, string: &str, aliases: &mut Vec<Alias>) {
    let alias = Alias::new(name.to_owned(), string.to_owned());
    aliases.push(alias);
}

fn edit_alias(name: &str, string: &str, aliases: &mut Vec<Alias>) {
    remove_alias(name, aliases);
    add_alias(name, string, aliases);
}

fn get_aliases(config: &Config) -> Result<Vec<Alias>> {
    let aliases_file = File::open(&config.aliases_path)?;
    let mut aliases: Vec<Alias> = vec![];
    let reader = BufReader::new(aliases_file);
    for result in reader.lines() {
        let line = result?;
        let alias = Alias::try_from(line)?;
        aliases.push(alias);
    }
    Ok(aliases)
}

fn remove_alias(name: &str, aliases: &mut Vec<Alias>) {
    aliases.retain(|a| a.name != name);
}

fn get_config() -> Result<Config> {
    let path = Path::new(env!("HOME")).join(".config/alias-manager/config.toml");
    Config::from(path)
}

fn set_aliases(aliases: &mut Vec<Alias>, config: Config) -> Result<()> {
    todo!()
}

// commands

fn add(name: &str, string: &str) -> Result<()> {
    let config = get_config()?;
    let aliases = &mut get_aliases(&config)?;
    add_alias(name, string, aliases);
    set_aliases(aliases, config)?;
    Ok(())
}

fn edit(name: &str, string: &str) -> Result<()> {
    let config = get_config()?;
    let aliases = &mut get_aliases(&config)?;
    edit_alias(name, string, aliases);
    set_aliases(aliases, config)?;
    Ok(())
}

fn list() -> Result<Vec<Alias>> {
    let config = get_config()?;
    let aliases = get_aliases(&config)?;
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["Name", "String"]);
    for alias in &aliases {
        table.add_row(row![alias.name, alias.string]);
    }
    table.printstd();
    Ok(aliases)
}

fn remove(name: &str) -> Result<()> {
    let config = get_config()?;
    let aliases = &mut get_aliases(&config)?;
    remove_alias(name, aliases);
    set_aliases(aliases, config)?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { name, string } => {
            add(name, string).unwrap();
        }
        Commands::Edit { name, string } => {
            edit(name, string).unwrap();
        }
        Commands::List => {
            list().unwrap();
        }
        Commands::Remove { name } => {
            remove(name).unwrap();
        }
    }
}
