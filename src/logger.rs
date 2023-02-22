pub(crate) use crate::cli::Args;
use std::path::Path;
use tracing_appender::{self, non_blocking::WorkerGuard};
use tracing_subscriber::{self, filter::LevelFilter, fmt::format::FmtSpan, prelude::*, Registry};

/// Initialize tracing with optional file logging
pub fn setup_tracing(args: &Args) -> Option<WorkerGuard> {
    // Initialize layer for console logging
    let tracing_stdout_layer = tracing_subscriber::fmt::layer()
        //.with_thread_ids(true)
        .with_span_events(FmtSpan::FULL)
        .with_filter(LevelFilter::from_level(match args.verbose {
            0 => tracing::Level::INFO,
            1 => tracing::Level::DEBUG,
            _ => tracing::Level::TRACE,
        }));

    // Optionally initialize layer for file logging
    let (tracing_file_layer, tracing_file_guard) = match args.log_file {
        None => (None, None),
        Some(ref log_file) => setup_file_logging(log_file, args.verbose + 1),
    };

    // Create the tracing subscriber
    let subscriber = Registry::default()
        .with(tracing_stdout_layer)
        .with(tracing_file_layer);

    // Set the global tracing subscriber
    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to initialize tracing global default subscriber");

    tracing_file_guard
}

/// Initialize file logging
fn setup_file_logging<S>(
    log_file: &str,
    verbose: u8,
) -> (
    Option<Box<dyn tracing_subscriber::Layer<S> + Send + Sync + 'static>>,
    Option<WorkerGuard>,
)
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    let path = Path::new(log_file);

    // Create a file appender for a valid log file path
    let file_appender = match (path.parent(), path.file_name()) {
        (Some(d), Some(f)) => Some(tracing_appender::rolling::never(d, Path::new(f))),
        _ => None,
    };

    // Create the file logging layer
    match file_appender {
        None => (None, None),
        Some(fa) => {
            let (file_writer, guard) = tracing_appender::non_blocking(fa);
            let file_layer = tracing_subscriber::fmt::layer()
                .with_ansi(false) // Disable ANSI colors
                .with_thread_ids(true)
                .with_span_events(FmtSpan::FULL)
                .with_writer(file_writer)
                .with_filter(LevelFilter::from_level(match verbose {
                    0 => tracing::Level::INFO,
                    1 => tracing::Level::DEBUG,
                    _ => tracing::Level::TRACE,
                }))
                .boxed();
            (Some(file_layer), Some(guard))
        }
    }
}
