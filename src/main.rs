//! Novel Writer Application
//! 
//! A professional novel writing tool built with Rust and Dioxus.
//! 
//! # Architecture
//! - `init`: Application initialization and setup
//! - `config`: Configuration management
//! - `db`: Database layer
//! - `core`: Business logic
//! - `ui`: User interface
//! - `utils`: Utility functions

use novel_writer::init;
use novel_writer::ui::App;
use dioxus::prelude::*;
use log::error;

fn main() {
    // Initialize application
    if let Err(e) = init::initialize_app() {
        error!("Failed to initialize application: {}", e);
        eprintln!("Failed to initialize application. Please check the logs for details.");
        std::process::exit(1);
    }
    
    // Launch UI
    launch(App);
}