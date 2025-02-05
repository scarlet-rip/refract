use refract_sc::run_eframe;
use tracing::{subscriber, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> eframe::Result {
    setup_tracing_subscriber();

    run_eframe()?;

    Ok(())
}

fn setup_tracing_subscriber() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    subscriber::set_global_default(subscriber)
        .expect("failed to set the default subscriber failed");
}
