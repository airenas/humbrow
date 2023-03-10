use std::io;

use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;

pub async fn do_zombie_reaper(cancel_token: CancellationToken) -> Result<(), String> {
    log::info!("hook to sigchild");
    let mut child_stream = signal(SignalKind::child()).unwrap();
    loop {
        tokio::select! {
            _ = child_stream.recv() => {

                log::info!("Reap");
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
        let options = 0x1;
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
