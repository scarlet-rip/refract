use tracing::{subscriber, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> eframe::Result {
    setup_tracing_subscriber();

    refract_sc::start_trackers().await;

    Ok(())
}

fn setup_tracing_subscriber() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    subscriber::set_global_default(subscriber)
        .expect("failed to set the default subscriber failed");
}
