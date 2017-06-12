use ToBlock;
use chrono::Local;
use errors::*;
use protocol::{Block, BlockBuilder};

pub struct DateTime {
    format: String,
}

impl DateTime {
    pub fn new(format: &str) -> Box<DateTime> {
        Box::new(DateTime { format: format.to_string() })
    }

    pub fn default() -> Box<DateTime> {
        DateTime::new("%d.%m.%Y %H:%M")
    }
}

impl ToBlock for DateTime {
    fn to_block(&self) -> Result<Block> {
        let now = Local::now().format(&self.format).to_string();
        Ok(BlockBuilder::default().full_text(now).build()?)
    }
}
