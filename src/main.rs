#![cfg_attr(not(debug_assertions), windows_subsystem="windows")]
#![allow(rustdoc::missing_crate_level_docs)]

use eframe::egui::{
    self,
    Color32,
    Event,
    Slider,
    Vec2
};

use egui_plot::{
    Legend,
    Line,
    Plot, 
    PlotPoint,
    PlotPoints,
};

fn main() {
    println!("Hello, world!");
}
