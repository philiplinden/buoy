use std::io;
use tracing_subscriber::{prelude::*, EnvFilter, fmt::format::FmtSpan};
use tracing_subscriber::fmt::time::UtcTime;

/// Configure the global tracing subscriber with a custom format and filtering.
///
/// This function sets up a tracing subscriber that:
/// - Uses a custom format with timestamps and spans
/// - Filters logs based on the RUST_LOG environment variable
/// - Includes file and line numbers in the output
/// - Shows spans for better context
/// - Uses colors when outputting to a terminal
pub fn configure_logging() -> Result<(), Box<dyn std::error::Error>> {
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info,buoy=debug"))?;

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_timer(UtcTime::rfc_3339())
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_span_events(FmtSpan::NEW)
        .with_ansi(atty::is(atty::Stream::Stdout))
        .with_writer(io::stdout);

    let subscriber = tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

/// Configure a development-specific tracing subscriber with more verbose output.
///
/// This function sets up a tracing subscriber that:
/// - Shows all debug and trace level logs
/// - Includes more detailed span information
/// - Uses a more verbose format suitable for development
pub fn configure_dev_logging() -> Result<(), Box<dyn std::error::Error>> {
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("debug,buoy=trace"))?;

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_timer(UtcTime::rfc_3339())
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_span_events(FmtSpan::NEW)
        .with_ansi(atty::is(atty::Stream::Stdout))
        .with_writer(io::stdout);

    let subscriber = tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{debug, error, info, warn};

    #[test]
    fn test_logging_configuration() {
        configure_logging().unwrap();
        
        // Test different log levels
        error!("This is an error message");
        warn!("This is a warning message");
        info!("This is an info message");
        debug!("This is a debug message");
    }

    #[test]
    fn test_dev_logging_configuration() {
        configure_dev_logging().unwrap();
        
        // Test different log levels
        error!("This is an error message");
        warn!("This is a warning message");
        info!("This is an info message");
        debug!("This is a debug message");
    }
} 