#![feature(plugin, trace_macros)]

#![plugin(clippy)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate nom;
extern crate env_logger;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod errors;
#[allow(dead_code)]
pub mod parser;
pub mod lexer;

use errors::*;
use lexer::{Lexer, Token};

fn main() {
    env_logger::init().unwrap();

    if let Err(e) = parse_args().and_then(run) {
        eprintln!("error: {}", e);
        for e in e.iter().skip(1) {
            eprintln!("caused by: {}", e);
        }
        // if let Some(backtrace) = e.backtrace() {
        //     eprintln!("backtrace: {:?}", backtrace);
        // }
    }
}

#[allow(needless_pass_by_value)]
fn run(source: Vec<u8>) -> Result<()> {
    let mut lexer = Lexer::new(&source);
    loop {
        match lexer.next_token() {
            Token::EOF => {
                debug!("EOF");
                break;
            }
            token => debug!("{:?}", token),
        }
    }
    Ok(())
}

fn parse_args() -> Result<Vec<u8>> {
    let file_name = env::args()
        .nth(1)
        .ok_or("No file provided")?;

    let mut file = File::open(Path::new(&file_name))
        .chain_err(|| "Could not open file")?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .chain_err(|| "Failed reading from file")?;

    Ok(buf)
}
