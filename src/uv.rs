///# `QuickTOCuv`  MIO - hardware,simulation
/// 
///   * Ultra
///   * Nitritox
///   * Loop
///   * Biomonitor
/// 
// use std::prelude::*;
// use std::io;

// use std::stream::Stream;
use std::path::{PathBuf};
use std::convert::{TryFrom};

// use std::stream;
// use std::time::Duration;
// use std::prelude::*;

// pub const VELOCITY: &'static str = "velocity";
// pub const POSITION: &'static str = "position";
// pub const HOLD: &'static str = "hold";
// pub const MAX: &'static str = "max";
// pub const ENDSCHALTER: &'static str = "endschalter";
// pub const PTP: &'static str = "ptp";
// pub const COMMAND: &'static str = "command";
// pub const GOTO: &'static str = "goto";

use super::*;


pub struct Uv {
    path:       PathBuf,
    pub lamp:   Lamp,
    pub sampl:  Vec<Valve>,
    pub cal:    Valve,
    pub tic:    Valve,
    pub bypass: Valve,
    pub gp:     GearPump,
    pub ndir:   Vec<Sensor>,
}


pub fn setup_hardware(station:&mut Workspace) -> Result<Uv> {
    let lamp   = Lamp::try_from(station.create_interface("lamp")?)?;
    let sv1    = Valve::try_from(station.create_interface("valve1")?)?;
    let sv2    = Valve::try_from(station.create_interface("valve2")?)?;
    let sv3    = Valve::try_from(station.create_interface("valve3")?)?;
    let sv4    = Valve::try_from(station.create_interface("valve4")?)?;
    let sv5    = Valve::try_from(station.create_interface("valve5")?)?;
    let sv6    = Valve::try_from(station.create_interface("valve6")?)?;
    let cal    = Valve::try_from(station.create_interface("cal")?)?;
    let tic    = Valve::try_from(station.create_interface("tic")?)?;
    let bypass = Valve::try_from(station.create_interface("bypas")?)?;
    let gp     = GearPump::try_from(station.create_interface("gp")?)?;
    let ndir1  = Sensor::try_from(station.create_interface("ndir1")?)?;
    let ndir2  = Sensor::try_from(station.create_interface("ndir2")?)?;
    let sampl  = vec![sv1, sv2 ,sv3, sv4 ,sv5, sv6];
    let ndir   = vec![ndir1,ndir2];
    let path   = station.path.to_path_buf();
    Ok(Uv{path,lamp,sampl,cal,tic,bypass,gp,ndir})
}



pub fn setup_methode(station:&Workspace)  {

}

impl Uv {

}