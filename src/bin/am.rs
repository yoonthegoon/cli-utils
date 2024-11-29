use cli_utils::error::Error;
use cli_utils::result::Result;
use regex::Regex;

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
        let name = caps.name("name").ok_or(Error::OptionNone)?.as_str().to_owned();
        let string = caps.name("string").ok_or(Error::OptionNone)?.as_str().to_owned();
        Ok(Alias::new(name, string))
    }
}

fn add(name: &str, string: &str) -> Result<String> {
    todo!()
}

fn edit(name: &str, string: &str) -> Result<()> {
    todo!()
}

fn list() -> Result<Vec<Alias>> {
    todo!()
}

fn remove(name: &str) -> Result<()> {
    todo!()
}

fn main() {}
