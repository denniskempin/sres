use std::sync::Once;

static ONCE_INIT: Once = Once::new();

pub fn init() {
    ONCE_INIT.call_once(|| {
        env_logger::builder()
            .filter_level(log::LevelFilter::Trace)
            .format_timestamp(None)
            .format_target(false)
            .init();

        // run initialization here
    });
}
