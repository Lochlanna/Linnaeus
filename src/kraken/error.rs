use std::fmt::{Display, Formatter};
use anyhow::bail;
use std::str::FromStr;
use strum::{EnumString, Display};
use serde::Deserialize;
use std::string::ToString;

#[derive(Debug, EnumString, Deserialize, Display, Copy, Clone)]
pub enum Severity {
    #[strum(serialize = "w", serialize = "W")]
    Warning,
    #[strum(serialize = "e", serialize = "E")]
    Error
}

#[derive(Debug, EnumString, Deserialize, Display, Copy, Clone)]
pub enum Category {
    General,
    Auth,
    API,
    Query,
    Order,
    Trade,
    Funding,
    Service
}


#[derive(Debug, Deserialize, Clone)]
pub struct KrakenError {
    severity: Severity,
    category: Category,
    message: String,
    additional: Option<String>
}

impl Display for KrakenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let formatted: String = if let Some(additional) = &self.additional {
            format!("{}{}:{}:{}", self.severity, self.category, self.message, additional)
        } else {
            format!("{}{}:{}", self.severity, self.category, self.message)
        };
        f.write_str(formatted.as_str())
    }
}

impl std::error::Error for KrakenError {

}

impl FromStr for KrakenError {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts:Vec<&str> = s.split(':').collect();
        if parts.len() < 2 || parts.len() > 2 {
            bail!("Malformed kraken error string. Expecting 2 or 3 parts got {}. Error string is {}", parts.len(), s);
        }
        let (severity_str, category_str) = parts[0].split_at(1);
        let severity = match Severity::from_str(severity_str) {
            Ok(s) => s, Err(_) => bail!("Malformed kraken error string. Expecting first char to be 'W' or 'E' got {}. Error string is {}", severity_str, s)
        };
        let category = match Category::from_str(category_str) {
            Ok(s) => s, Err(_) => bail!("Malformed kraken error string. Unknown category {}. Error string is {}", category_str, s)
        };
        let message = parts[1].to_string();
        let additional = parts.get(2).map(|a| a.to_string());
        Ok(KrakenError {
            severity,
            category,
            message,
            additional
        })
    }
}

impl From<&str> for KrakenError {
    fn from(input: &str) -> Self {
        match KrakenError::from_str(input) {
            Ok(ke) => ke,
            Err(e) => panic!("Cannot convert input to kraken error. Input is {}, error is {}", input, e)
        }
    }
}

impl From<String> for KrakenError {
    fn from(input: String) -> Self {
        match KrakenError::from_str(&input) {
            Ok(ke) => ke,
            Err(e) => panic!("Cannot convert input to kraken error. Input is {}, error is {}", input, e)
        }
    }
}