/// Mio  cli 
// use std::io;
// use std::io::prelude::*;
use std::path::{PathBuf};
use structopt::StructOpt;





/// ✇ average signal
#[derive(Debug, StructOpt)]
pub struct Setup {
    ///🗁 miofs directory to link hardware moduls 
    #[structopt(long = "miofs", about = "📢 miofs directory - link driver", default_value = "/run/user/1000")]
    path: PathBuf,
}


/// 📢 subcommands
#[derive(Debug, StructOpt)]
pub enum Cmd {
   #[structopt(name = "watch", about = "📢 watch ws mio directory")]
    Watch,
    #[structopt(name = "new", about = "📢 subcommand to create new working directory")]
    New(Setup),
    #[structopt(name = "clean", about = "📢 subcommand to clean working directory")]
    Clean,

}

/// 📢 multi io subcommand to run.
#[derive(Debug, StructOpt)]
#[structopt(name = "miofs", about = "  🧰 miofs interface usage.")]
pub struct Args {
    // path: PathBuf,
    ///🔌 Command
    #[structopt(subcommand, about = "📢 subcommand to serve controller or start pipeline directly")]
    cmd:Cmd,

}

//
impl Args {
   // Access the directory name.
}
