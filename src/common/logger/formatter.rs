use std::fmt;

use chrono::Local;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::{
    fmt::{
        FmtContext,
        format::{FormatEvent, FormatFields, Writer},
    },
    registry::LookupSpan,
};

const RESET: &str = "\x1b[0m";

const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const GRAY: &str = "\x1b[90m";
const PURPLE: &str = "\x1b[38;2;167;139;250m"; // RGB 紫色

pub struct CustomLogFormat {
    pub target: String,

    /// 是否启用终端 ANSI 颜色
    pub ansi: bool,
}

impl CustomLogFormat {
    fn level_color(&self, level: &Level) -> &'static str {
        if !self.ansi {
            return "";
        }

        match *level {
            Level::TRACE => MAGENTA,
            Level::DEBUG => BLUE,
            Level::INFO => GREEN,
            Level::WARN => YELLOW,
            Level::ERROR => RED,
        }
    }

    fn color(&self, color: &'static str) -> &'static str {
        if self.ansi { color } else { "" }
    }

    fn reset(&self) -> &'static str {
        if self.ansi { RESET } else { "" }
    }
}

impl<S, N> FormatEvent<S, N> for CustomLogFormat
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    N: for<'writer> FormatFields<'writer> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let metadata = event.metadata();

        let time = Local::now().format("%Y-%m-%d %H:%M:%S");
        let file = metadata.file().unwrap_or("unknown");
        let line = metadata.line().unwrap_or(0);

        let reset = self.reset();

        write!(writer, "{}[{}]{} ", self.color(CYAN), self.target, reset,)?;
        write!(
            writer,
            "{}[{}]{} ",
            self.level_color(metadata.level()),
            metadata.level(),
            reset,
        )?;
        write!(writer, "{}{}{} ", self.color(GRAY), time, reset,)?;
        write!(
            writer,
            "[{}{}{}:{}{}{}] ",
            self.color(PURPLE),
            file,
            reset,
            self.color(MAGENTA),
            line,
            reset,
        )?;
        write!(writer, "{} | {} ", self.color(GRAY), reset,)?;
        // 实际 message 和结构化字段
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}
