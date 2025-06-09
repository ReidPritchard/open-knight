// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn init_logger() {
    Builder::new()
        .filter_level(LevelFilter::max())
        .filter_module("sea_orm", LevelFilter::Off)
        .filter_module("sqlx", LevelFilter::Off)
        .filter_module("tracing", LevelFilter::Off)
        .filter_module("tao", LevelFilter::Off)
        .init();
}

fn main() {
    init_logger();
    info!("Starting Open Knight");

    open_knight_lib::run();
}
