mod config;
mod data;
mod errors;
mod handlers;

use clap::Arg;
use data::Service;
use std::process;
use std::{error::Error, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use tokio_util::sync::CancellationToken;
use warp::Filter;

use clap::Command;
use config::Config;
use tokio::signal::unix::{signal, SignalKind};

use crate::handlers::CookieParams;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let cfg = app_config().unwrap_or_else(|err| {
        log::error!("problem parsing arguments: {err}");
        process::exit(1)
    });

    log::info!("Starting HUMan BROWser wrapper service");
    log::info!("Version      : {}", cfg.version);
    log::info!("Port         : {}", cfg.port);
    log::info!("Python       : {}", cfg.python);
    log::info!("Agent script : {}", cfg.agent_script);
    log::info!("Cookie script: {}", cfg.cookie_script);

    let cancel_token = CancellationToken::new();

    let ct = cancel_token.clone();

    tokio::spawn(async move {
        let mut int_stream = signal(SignalKind::interrupt()).unwrap();
        let mut term_stream = signal(SignalKind::terminate()).unwrap();
        tokio::select! {
            _ = int_stream.recv() => log::info!("Exit event int"),
            _ = term_stream.recv() => log::info!("Exit event term"),
            // _ = rx_exit_indicator.recv() => log::info!("Exit event from some loader"),
        }
        log::debug!("sending exit event");
        ct.cancel();
        log::debug!("expected drop tx_close");
    });

    let srv = Arc::new(RwLock::new(Service {
        user_agent: "".to_string(),
        exec_guard: Mutex::new(0),
        python: cfg.python.clone(),
        cookie_script: cfg.cookie_script,
    }));

    // let mut srv = Arc::new(Mutex::new("".to_string()));
    let srv_cl = srv.clone();

    let live_route = warp::get()
        .and(warp::path("live"))
        .and_then(handlers::live_handler);
    let agent_route = warp::get()
        .and(warp::path("agent"))
        .and(warp::header("user-agent"))
        .and(with_service(srv_cl))
        .and_then(handlers::agent_handler);
    let cookie_route = warp::get()
        .and(warp::path("cookie"))
        .and(warp::query::<CookieParams>())
        .and(with_service(srv))
        .and_then(handlers::cookie_handler);
    let routes = live_route
        .or(cookie_route)
        .or(agent_route)
        .with(warp::cors().allow_any_origin())
        .recover(errors::handle_rejection);

    let (_, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], cfg.port), async move {
            cancel_token.cancelled().await;
        });

    tokio::spawn(async move {
        log::info!("call to agent script");
        let url = format!("localhost:{}/agent", cfg.port);
        log::debug!("invoke : {} {} {}", cfg.python, cfg.agent_script, url);
        let output = std::process::Command::new(cfg.python.as_str()) // todo: add timeout for command
            .arg(cfg.agent_script.as_str())
            .arg(url.as_str())
            .output();
        match output {
            Ok(output) => {
                if output.status.success() {
                    log::info!("done");
                    log::debug!(
                        "Output:\n{}",
                        String::from_utf8_lossy(&output.stderr).into_owned()
                    );
                } else {
                    log::error!("{}", String::from_utf8_lossy(&output.stderr).into_owned());
                }
            }
            Err(err) => {
                log::error!("{}", err);
            }
        };
    });

    log::info!("wait for server finish");
    tokio::task::spawn(server).await.unwrap_or_else(|err| {
        log::error!("{err}");
        process::exit(1);
    });

    log::info!("Bye");
    Ok(())
}

fn with_service(
    srv: Arc<RwLock<Service>>,
) -> impl Filter<Extract = (Arc<RwLock<Service>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || srv.clone())
}

fn app_config() -> Result<Config, String> {
    let app_version = option_env!("CARGO_APP_VERSION").unwrap_or("dev");

    let cmd = Command::new("humbrow")
        .version(app_version)
        .author("Airenas V.<airenass@gmail.com>")
        .about("Service for bypassing cloudflare protection")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Service port")
                .env("PORT")
                .default_value("8000"),
        )
        .get_matches();
    let mut config = Config::build(&cmd)?;
    config.version = app_version.into();
    Ok(config)
}
