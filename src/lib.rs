#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate chrono;

pub mod errors;
pub mod protocol;
pub mod widgets;

use errors::*;
use std::fmt;
use std::thread::sleep;
use std::time::Duration;

pub trait ToBlock {
    fn to_block(&self) -> Result<protocol::Block>;
}

pub struct Status {
    refresh: Duration,
    blocks: Vec<Box<ToBlock>>,
}

impl Default for Status {
    fn default() -> Status {
        Status {
            refresh: Duration::from_secs(1),
            blocks: Vec::new(),
        }
    }
}

impl Status {
    pub fn new(refresh: Duration) -> Status {
        Status {
            refresh,
            blocks: Vec::new(),
        }
    }

    pub fn add(&mut self, block: Box<ToBlock>) -> &mut Self {
        let mut new = self;
        new.blocks.push(block);
        new
    }

    pub fn header(&self) -> Result<protocol::Header> {
        Ok(protocol::HeaderBuilder::default().version(1).build()?)
    }

    pub fn run(&self) -> Result<()> {
        println!("{}", self.header()?);
        println!("[");
        loop {
            println!("{},", self);
            sleep(self.refresh);
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (idx, block) in self.blocks.iter().enumerate() {
            let block = block.to_block().map_err(|_| fmt::Error)?;
            if idx == 0 {
                write!(f, "{}", block)?;
            } else {
                write!(f, ",{}", block)?;
            }
        }
        write!(f, "]")
    }
}
