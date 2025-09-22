use refract_sc::{run_frontend, run_gui, setup_tracing};
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() -> eframe::Result {
    setup_tracing();

    spawn_blocking(|| {
        run_frontend();
    });

    run_gui()?;

    Ok(())
}
