use std::{io, panic};
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Initialize tracing:
/// - Default: JSON to stdout (best for CloudWatch).
/// - When LOG_FORMAT=pretty -> human-friendly console output.
/// - Level comes from RUST_LOG (e.g. "info,mycrate=debug").
pub fn init_tracing() {
    // Default filter if RUST_LOG not set.
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,actix_web=info"));
    let format = std::env::var("LOG_FORMAT").unwrap_or_else(|_| "json".into());

    if format == "pretty" {
        tracing_subscriber::registry().with(filter).with(
            fmt::layer()
                .with_writer(io::stdout)        // stdout only
                .with_ansi(true)
                .with_span_events(FmtSpan::CLOSE)
                .with_target(false)              // cleaner output
                .with_timer(fmt::time::UtcTime::rfc_3339()),
        ).init();

        return;
    }

    tracing_subscriber::registry().with(filter).with(
        fmt::layer()
            .with_writer(io::stdout)        // stdout only
            .json()
            .with_span_events(FmtSpan::CLOSE)
            .with_current_span(true)
            .with_span_list(true)
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_timer(fmt::time::UtcTime::rfc_3339()),
    ).init();
}

pub fn install_panic_hook() {
    // Capture panics and log them through `tracing` so they show in CloudWatch.
    panic::set_hook(Box::new(|info| {
        // Extract panic message
        let msg = if let Some(s) = info.payload().downcast_ref::<&str>() {
            *s
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.as_str()
        } else {
            "panic payload not a string"
        };

        let (file, line, col) = info.location()
            .map(|l| (l.file(), l.line(), l.column()))
            .unwrap_or(("<unknown>", 0, 0));

        // Capture a backtrace (force capture regardless of env)
        let backtrace = std::backtrace::Backtrace::force_capture().to_string();

        tracing::error!(
            target: "panic",
            message = %msg,
            file = %file,
            line = line,
            column = col,
            backtrace = %backtrace,
            "application panic"
        );
    }));
}