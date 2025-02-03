use fern::Dispatch;
use log::LevelFilter;

pub fn setup_logger() {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}