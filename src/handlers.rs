use std::{process::Command, sync::Arc};

use crate::{data::Service, errors::ParamError};
use serde::Serialize;
use serde_derive::Deserialize;
use tokio::sync::RwLock;
use warp::{http::StatusCode, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;

#[derive(Deserialize)]
pub struct CookieParams {
    url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CookieResult {
    agent: String,
    cookie: String,
}

pub async fn live_handler() -> Result<impl Reply> {
    log::debug!("live handler");
    Ok(warp::http::Response::builder()
        .status(StatusCode::OK)
        .body("OK".to_string()))
}

pub async fn cookie_handler(
    params: CookieParams,
    srv_wrap: Arc<RwLock<Service>>,
) -> Result<impl Reply> {
    log::debug!("cookie handler");
    let srv = srv_wrap.read().await;
    let mut count = srv.exec_guard.lock().await; // lock for only one worker at a time
    *count += 1;
    log::debug!("invocations: {count}");
    log::debug!("user-agent : {}", srv.user_agent);
    let mut url = params.url.unwrap_or_default();
    if url.is_empty() {
        return Err(ParamError {
            msg: "No target url".to_string(),
        }
        .into());
    }
    if !url.starts_with("http") {
        url = "https://".to_owned() + url.as_str();
    }

    let _reap_guard = srv.reap_guard.read().await; // prevent reaping child process while we are reading from the process
    log::debug!("invoke : {} {} {}", srv.python, srv.cookie_script, url);
    let output = Command::new(srv.python.as_str()) //todo add timetout for command
        .arg(srv.cookie_script.as_str())
        .arg(url.as_str())
        .output();
    let cmd_res = match output {
        Ok(output) => {
            if output.status.success() {
                log::debug!(
                    "Output:\n{}",
                    String::from_utf8_lossy(&output.stderr).into_owned()
                );
                Ok(String::from_utf8_lossy(&output.stdout).into_owned())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).into_owned())
            }
        }
        Err(err) => Err(err.to_string()),
    };
    match cmd_res {
        Ok(s) => {
            let res = CookieResult {
                agent: srv.user_agent.clone(),
                cookie: String::from(s.trim()),
            };
            Ok(warp::reply::json(&res).into_response())
        }
        Err(err) => Err(ParamError { msg: err }.into()),
    }
}

pub async fn agent_handler(agent: String, srv: Arc<RwLock<Service>>) -> Result<impl Reply> {
    log::debug!("agent handler");
    let mut srv = srv.write().await;
    log::debug!("old user agent: {}", srv.user_agent);
    log::debug!("new user agent: {agent}");
    srv.user_agent = agent;
    Ok(warp::http::Response::builder()
        .status(StatusCode::OK)
        .body("OK"))
}
