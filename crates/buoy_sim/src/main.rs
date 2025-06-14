mod logging;

use tracing::info;

fn main() {
    // Configure logging based on the environment
    if cfg!(debug_assertions) {
        if let Err(e) = logging::configure_dev_logging() {
            eprintln!("Failed to configure development logging: {}", e);
            return;
        }
    } else if let Err(e) = logging::configure_logging() {
        eprintln!("Failed to configure logging: {}", e);
        return;
    }

    info!("Starting Buoy simulation");
    
    // TODO: Initialize Bevy app and run simulation
    info!("Simulation complete");
}
