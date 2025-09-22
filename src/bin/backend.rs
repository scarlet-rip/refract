use refract_sc::{run_backend, setup_tracing};

#[tokio::main]
async fn main() -> eframe::Result {
    setup_tracing();

    run_backend().await;

    Ok(())
}
