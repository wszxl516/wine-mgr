use env_logger::{fmt::Color, Builder};
use log::Level;
use std::io::Write;
pub fn init_logger() {
    let env = env_logger::Env::default()
        .filter_or("log", "info")
        .write_style_or("log", "always");
    Builder::from_env(env)
        .format(|buf, record| {
            let mut style = buf.style();
            let color = match record.level() {
                Level::Error => Color::Red,
                Level::Warn => Color::Yellow,
                Level::Info => Color::Green,
                Level::Debug => Color::Blue,
                Level::Trace => Color::Magenta,
            };
            style.set_color(color).set_intense(false);
            let timestamp = buf.timestamp();
            writeln!(
                buf,
                "[{} {} {}]: {}",
                style
                    .clone()
                    .set_intense(true)
                    .set_color(Color::Rgb(100, 100, 100))
                    .set_bold(true)
                    .value("WINE: "),
                timestamp,
                style
                    .clone()
                    .set_intense(true)
                    .set_bold(true)
                    .value(record.level()),
                style.value(record.args())
            )
        })
        .init();
}
