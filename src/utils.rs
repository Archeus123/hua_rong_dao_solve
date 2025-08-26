pub fn init_log() {
    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::{Appender, Config, Root};
    use log4rs::encode::pattern::PatternEncoder;

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%y-%m-%d %H:%M:%S%.3f)} {h({level}):5} {I} [{T}] {t} -- {m}{n}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .unwrap();

    log4rs::init_config(config).unwrap();

    // log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    std::panic::set_hook(Box::new(|info| {
        let msg = if let Some(s) = info.payload().downcast_ref::<&'static str>() {
            s
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s
        } else {
            "Box<Any>"
        };
        if let Some(location) = info.location() {
            log::error!("{} ({}:{})", msg, location.file(), location.line());
        } else {
            log::error!("{} (unknown)", msg);
        }
    }));
}

#[macro_export]
macro_rules! log_guard {
    ($v:expr) => {{
        let v = $v;
        if v.is_err() {
            log::error!("{}", v.err().unwrap());
            return;
        } else {
            v.unwrap()
        }
    }};
}
