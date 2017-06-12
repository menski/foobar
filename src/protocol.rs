use serde_json;
use std::fmt;

#[derive(Debug, Default, PartialEq, Serialize, Builder)]
#[builder(default)]
pub struct Header {
    version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_signal: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cont_signal: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    click_events: Option<bool>,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let json = serde_json::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", json)
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Builder)]
#[builder(default)]
pub struct Block {
    full_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    urgent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    separator: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    separator_block_width: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    markup: Option<String>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let json = serde_json::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", json)
    }
}
