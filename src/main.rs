// Application to get coordinates and distance of cities using API, CSV or Mock data
// Authors: Lucas Carranza, Adrian Céspedes

mod city;
mod city_api;
mod city_csv;
mod city_interface;
mod city_mock;
mod coords;
mod distance;
mod gui;
mod three_point;

use gui::MyApp;

fn main() {
    let native_methods = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "City Distance Calculator",
        native_methods,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}
