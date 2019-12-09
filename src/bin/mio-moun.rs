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


/// 📢 driver select
#[derive(Debug, StructOpt)]
pub enum Driver {
    #[structopt(name = "simulate", about = "📢 simulation driver")]
    Simulate,
    #[structopt(name = "dbus", about = "📢 setup dbus driver")]
    DBus{
        ///🔌 hardware connection address
        #[structopt(short = "a", long = "address",  default_value = "tcp:host=192.168.66.59,port=6666")]
        address: String,
    },
}

/// 📢 multi io subcommand to run.
#[derive(Debug, StructOpt)]
#[structopt(name = "miofs", about = "  🧰 miofs interface usage.")]
pub struct Args {
    ///🗁 miofs directory
    #[structopt(short = "p", long = "path", about = "📢 simulation driver", default_value = "/pwa")]
    path: PathBuf,
    ///🔌 driver
    #[structopt(subcommand, about = "📢 subcommand to serve controller or start pipeline directly")]
    driver:Driver,
}

/// 🔧 activate debug mode
impl Args {
    /// Access the directory name.
    #[inline]
    pub fn command(&self) -> io::Result<()> {
               Ok(())
    }
    pub fn pid(&self) -> io::Result<PathBuf> {
        Ok(self.path.join("pid"))
    }
}


#[paw::main]
fn main(args: Args) -> io::Result<()> {
    femme::start(log::LevelFilter::Trace).unwrap();
    let pid = args.pid()?;
    let d =  mio::Mio::mount(&args.path.join("pwa"),"miofs");
    ctrlc::set_handler(move || {
        // let m = mio.
        process::abort();
    }).expect("Error setting Ctrl-C handler");
    loop {

    };
    Ok(())
}


