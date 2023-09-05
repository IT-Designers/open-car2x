use log::LevelFilter;
use log::SetLoggerError;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::Appender;
use log4rs::config::Config;
use log4rs::config::Logger;
use log4rs::config::Root;
use log4rs::encode::pattern::PatternEncoder;

pub fn init(package: &str, level: Option<LevelFilter>) -> Result<::log4rs::Handle, SetLoggerError> {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%Y-%m-%d %H:%M:%S%.3f)}  {M:>30.30}:{L:>03}  {T:>25.25}  {l:>5}  {m})}{n}",
        )))
        .build();

    //AppenderBuilder::build("tracker", Box::new())
    let level = level.unwrap_or(LevelFilter::Info);
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build(env!("CARGO_PKG_NAME"), level))
        .logger(Logger::builder().build(package, level))
        .build(Root::builder().appender("stdout").build(level))
        .expect("Failed to create logger config");

    ::log4rs::init_config(config)
}
