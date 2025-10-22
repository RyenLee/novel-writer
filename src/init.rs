/// Application initialization module
/// Handles database setup and application bootstrapping

use crate::db;
use log::{info, warn, error};

/// Initialize the application
/// 
/// This function performs all necessary setup operations:
/// - Database initialization
/// - Schema migration
/// - Configuration loading
pub fn initialize_app() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            use std::io::Write;
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            writeln!(buf, "[{} {} {}] {}", 
                timestamp, 
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Info)
        .init();
    
    info!("Starting Novel Writer application...");
    
    // Initialize database
    info!("Initializing database...");
    init_database()?;
    info!("Database initialized successfully");
    
    // Future: Add other initialization logic here
    // - Load user preferences
    // - Check for updates
    
    info!("Application initialization completed successfully");
    
    Ok(())
}

/// Initialize the database
/// 
/// Creates the database file if it doesn't exist and runs migrations
fn init_database() -> Result<(), Box<dyn std::error::Error>> {
    match crate::db::init_database() {
        Ok(()) => {
            info!("Database migration completed successfully");
            Ok(())
        },
        Err(e) => {
            error!("Database initialization error: {}", e);
            Err(format!("Failed to initialize database: {}", e).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_database_initialization() {
        // Note: This would need proper test database setup
        assert!(init_database().is_ok());
    }
}
