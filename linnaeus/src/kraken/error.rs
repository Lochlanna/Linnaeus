use anyhow::bail;
use std::str::FromStr;
use strum::EnumString;

#[derive(Debug, EnumString)]
pub enum Severity {
    #[strum(serialize = "w", serialize = "W")]
    Warning,
    #[strum(serialize = "e", serialize = "E")]
    Error
}

#[derive(Debug, EnumString)]
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


#[derive(Debug)]
pub struct KrakenError {
    severity: Severity,
    category: Category,
    message: String,
    additional: Option<String>
}

impl KrakenError {
    pub fn from_str(input: &str) -> anyhow::Result<KrakenError> {
        let parts:Vec<&str> = input.split(':').collect();
        if parts.len() < 2 || parts.len() > 2 {
            bail!("Malformed kraken error string. Expecting 2 or 3 parts got {}. Error string is {}", parts.len(), input);
        }
        let (severity_str, category_str) = parts[0].split_at(1);
        let severity = match Severity::from_str(severity_str) {
            Ok(s) => s, Err(_) => bail!("Malformed kraken error string. Expecting first char to be 'W' or 'E' got {}. Error string is {}", severity_str, input)
        };
        let category = match Category::from_str(category_str) {
            Ok(s) => s, Err(_) => bail!("Malformed kraken error string. Unknown category {}. Error string is {}", category_str, input)
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