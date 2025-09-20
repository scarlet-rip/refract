use refract_sc::run_eframe;
use tokio::task::spawn_blocking;
use tracing::{subscriber, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> eframe::Result {
    setup_tracing_subscriber();

    spawn_blocking(|| {
        refract_sc::start();
    });

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
