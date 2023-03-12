use std::{io, sync::Arc};

use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

pub async fn do_zombie_reaper(
    cancel_token: CancellationToken,
    reap_guard: Arc<RwLock<i32>>,
) -> Result<(), String> {
    log::debug!("hook to sigchild");
    let mut child_stream = signal(SignalKind::child()).map_err(|e| e.to_string())?;
    loop {
        tokio::select! {
            _ = child_stream.recv() => {
                log::debug!("sigchild event");
                let _reap_guard = reap_guard.write().await;
                reap().unwrap_or_else(|e| {log::error!("{e}");});
            },
            _ = cancel_token.cancelled() => break
        }
    }
    Ok(())
}

fn reap() -> Result<(), io::Error> {
    loop {
        let mut status = 0;
        let options = libc::WNOHANG;
        let mut rusage = std::mem::MaybeUninit::zeroed();
        let pid = unsafe { libc::wait4(-1, &mut status, options, rusage.as_mut_ptr()) };
        if pid > 0 {
            log::debug!("Reaped {pid}");
            continue;
        }
        if pid == 0 || pid == -1 {
            // no children
            return Ok(());
        }
        return Err(io::Error::last_os_error());
    }
}
