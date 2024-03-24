use tracing::info;

#[cfg(not(unix))]
pub async fn shutdown() -> anyhow::Result<()> {
    use tokio::signal::ctrl_c;
    ctrl_c().await.map_err(Into::into)
}

#[cfg(unix)]
pub async fn shutdown() -> anyhow::Result<()> {
    use tokio::signal::unix::{signal, SignalKind};
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigquit = signal(SignalKind::quit())?;
    tokio::select! {
        _ = sigint.recv() => {
            info!(signal = "SIGINT", "graceful_shutdown");
        },
        _ = sigterm.recv() => {
            info!(signal = "SIGTERM", "graceful_shutdown");
        },
        _ = sigquit.recv() => {
            info!(signal = "SIGQUIT", "graceful_shutdown");
        }
    };
    Ok(())
}
