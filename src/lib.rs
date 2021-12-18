pub mod http;
pub mod cfg;

use env_logger::fmt::Color;
pub use log::LevelFilter;


/**
 * 初始化日志
 */
pub fn logger_init(level: LevelFilter) {
    let env = env_logger::Env::default().filter_or("MY_LOG", level.as_str());
    // env_logger::init_from_env(env);
    use std::io::Write;
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            let local_time = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S");
            let mut style = buf.style();
            match record.level(){
                log::Level::Debug=>&style.set_color(Color::Green),
                log::Level::Warn=>&style.set_color(Color::Yellow),
                log::Level::Error=>&style.set_color(Color::Red),
                _=>&style
            };
            writeln!(
                buf,
                "{}",
                style.value(format!(
                    "[{}]-({}):{}",
                    local_time,
                    record.level(),
                    record.args()
                ))
            )
        })
        .init()
}