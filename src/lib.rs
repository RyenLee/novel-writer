//! Novel Writer Library
//! 
//! Core library for the Novel Writer application.
//! 
//! # Modules
//! 
//! - [`init`] - Application initialization and setup
//! - [`config`] - Configuration management
//! - [`db`] - Database layer for persistent storage
//! - [`core`] - Core business logic (managers, state, utilities)
//! - [`ui`] - User interface components built with Dioxus
//! - [`utils`] - General utility functions

pub mod init;
pub mod config;
pub mod db;
pub mod ui;
pub mod core;
pub mod utils;