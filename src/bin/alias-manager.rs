use clap::{Parser, Subcommand};
use cli_utils::error::Error;
use cli_utils::result::Result;
use prettytable::{format, row, Table};
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Alias {
    name: String,
    string: String,
}

impl Alias {
    fn new(name: &str, string: &str) -> Alias {
        let name = name.to_string();
        let string = string.to_string();
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
        let name = caps.name("name").ok_or(Error::OptionNone)?.as_str();
        let string = caps.name("string").ok_or(Error::OptionNone)?.as_str();
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

fn add_alias(name: &str, string: &str, aliases: &mut Vec<Alias>) {
    let alias = Alias::new(name, string);
    aliases.push(alias);
}

fn edit_alias(name: &str, string: &str, aliases: &mut Vec<Alias>) {
    remove_alias(name, aliases);
    add_alias(name, string, aliases);
}

fn remove_alias(name: &str, aliases: &mut Vec<Alias>) {
    aliases.retain(|a| a.name != name);
}

fn get_aliases() -> Result<Vec<Alias>> {
    let aliases_path = Path::new(env!("HOME")).join(".aliases.sh");
    let aliases_file = File::open(&aliases_path)?;
    let mut aliases: Vec<Alias> = vec![];
    let reader = BufReader::new(aliases_file);
    for result in reader.lines() {
        let line = result?;
        let alias = Alias::try_from(line)?;
        aliases.push(alias);
    }
    Ok(aliases)
}

fn set_aliases(aliases: &mut Vec<Alias>) -> Result<()> {
    let aliases_path = Path::new(env!("HOME")).join(".aliases.sh");
    aliases.sort_by(|a, b| a.name.cmp(&b.name));
    let mut alias_strings: Vec<String> = vec![];
    for alias in aliases.iter() {
        let name = &alias.name;
        let string = &alias.string;
        let alias = Alias::new(name, string);
        alias_strings.push(alias.into());
    }
    let aliases_string = alias_strings.join("\n");
    fs::write(&aliases_path, aliases_string)?;
    Ok(())
}

fn add(name: &str, string: &str) -> Result<()> {
    let aliases = &mut get_aliases()?;
    add_alias(name, string, aliases);
    set_aliases(aliases)?;
    let alias_string: String = Alias::new(name, string).into();
    println!("{}", alias_string);
    Ok(())
}

fn edit(name: &str, string: &str) -> Result<()> {
    let aliases = &mut get_aliases()?;
    edit_alias(name, string, aliases);
    set_aliases(aliases)?;
    let alias_string: String = Alias::new(name, string).into();
    println!("{}", alias_string);
    Ok(())
}

fn list() -> Result<Vec<Alias>> {
    let aliases = get_aliases()?;
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
    let aliases = &mut get_aliases()?;
    let len = aliases.len();
    remove_alias(name, aliases);
    set_aliases(aliases)?;
    if aliases.len() == len {
        return Err(Error::Generic("no alias removed".to_string()));
    }
    println!("unalias {}", name);
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
