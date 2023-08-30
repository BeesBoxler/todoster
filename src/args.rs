use std::io::stdout;

pub struct Args {
    pub format: Format,
}

impl Default for Args {
    fn default() -> Self {
        let format = if termion::is_tty(&stdout()) {
            Format::Tty
        } else {
            Format::Plain
        };
        Self { format }
    }
}

#[derive(Debug)]
pub struct ArgumentError(String);

#[derive(PartialEq)]
pub enum Format {
    Markdown,
    Plain,
    Tty,
}

impl TryFrom<String> for Format {
    type Error = ArgumentError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match &value.to_lowercase()[..] {
            "md" | "markdown" => Ok(Format::Markdown),
            "plain" => Ok(Format::Plain),
            unknown => Err(ArgumentError(format!(
                "Could not parse format: {}",
                unknown
            ))),
        }
    }
}
