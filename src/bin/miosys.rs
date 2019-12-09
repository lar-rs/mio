//! MultiIO
//! 
//!
//!
// use std::fs;
// use std::fs::File;
// use std::io::LineWriter;
use std::io;
// use std::io::prelude::*;
use std::path::{PathBuf};
use structopt::StructOpt;
// use clap_flags::Log;
use std::process;
// use std::stream;
// use std::prelude::*;
// use std::time;
// use can::error::CanError;
// use regex::{Regex,RegexSetBuilder};
// use regex::Regex;
// use lazy_static::lazy_static;
// use std::time::Duration;
// use std::str::FromStr;
// use std::time::SystemTime;
// use async_std::println;

use mio;
use mio::cli::Args;


#[paw::main]
fn main(args: Args) -> io::Result<()> {
    femme::start(log::LevelFilter::Trace).unwrap();

    
    let ws = mio::Workspace::create()?;
    ctrlc::set_handler(move || {
        // let m = mio.
        process::abort();
    }).expect("Error setting Ctrl-C handler");
    ws.mio.watch()?;
    // match args.cmd {

    // }
    Ok(())
}


