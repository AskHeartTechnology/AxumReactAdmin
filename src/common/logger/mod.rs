pub mod formatter;
pub mod http;
pub mod macros;

use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{common::logger::formatter::CustomLogFormat, config::AppConfig};

pub fn init(config: &AppConfig) {
    let filter = EnvFilter::try_new(&config.log.level).unwrap_or_else(|_| EnvFilter::new("info"));

    if config.log.format.eq_ignore_ascii_case("json") {
        tracing_subscriber::registry()
            .with(filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_current_span(true)
                    .with_span_list(true)
                    .flatten_event(true),
            )
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .event_format(CustomLogFormat {
                        target: config.log.target.clone(),
                        ansi: config.log.ansi,
                    })
                    .with_ansi(true),
            )
            .init();
    }
}
