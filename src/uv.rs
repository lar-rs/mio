///# `QuickTOCuv`  MIO - hardware,simulation
/// 
///   * Ultra
///   * Nitritox
///   * Loop
///   * Biomonitor
/// 
// use async_std::prelude::*;
// use async_std::io;

use super::{MioError};
use async_std::fs;
// use async_std::stream::Stream;
use async_std::path::{Path,PathBuf};
// use async_std::stream;
// use std::time::Duration;
// use async_std::prelude::*;

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
    pub path:   PathBuf,
    pub lamp:   Lamp,
    pub sv:     Vec<Valve>,
    pub cv:     Valve,
    pub tic:    Valve,
    pub bypass: Valve,
    pub gp1:    GearPump,
    pub ndir:   Vec<Sensor>,
}

pub async fn simulation(path:&Path) -> Result<Uv,MioError> {
    fs::create_dir_all(path).await?;
    let lamp   = lamp::create(path.join("lamp").as_path()).await?;
    let sv1    = valve::create(path.join("sv1").as_path()).await?;
    let sv2    = valve::create(path.join("sv2").as_path()).await?;
    let sv3    = valve::create(path.join("sv3").as_path()).await?;
    let sv4    = valve::create(path.join("sv4").as_path()).await?;
    let sv5    = valve::create(path.join("sv5").as_path()).await?;
    let sv6    = valve::create(path.join("sv6").as_path()).await?;
    let cv     = valve::create(path.join("cv1").as_path()).await?;
    let tic    = valve::create(path.join("ticv").as_path()).await?;
    let bypass = valve::create(path.join("bypass").as_path()).await?;
    let gp1    = pump::gearpump(path.join("gp1").as_path()).await?;
    let ndir1  = sensor::ndir(path.join("ndir1").as_path()).await?;
    let ndir2  = sensor::ndir(path.join("ndir2").as_path()).await?;
    let sv     = vec![sv1, sv2 ,sv3, sv4 ,sv5, sv6];
    let ndir   = vec![ndir1,ndir2];
    let path   = path.to_path_buf();
    Ok(Uv{path,lamp,sv,cv,tic,bypass,gp1,ndir})
}

