use serde_derive::Serialize;
use std::convert::Infallible;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Debug)]
pub struct ParamError {
    pub msg: String,
}
impl warp::reject::Reject for ParamError {}

#[derive(Debug)]
pub struct OtherError {
    pub msg: String,
}
impl warp::reject::Reject for OtherError {}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if err.find::<warp::filters::body::BodyDeserializeError>().is_some() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(e) = err.find::<ParamError>() {
        log::debug!("{}", e.msg);
        code = StatusCode::BAD_REQUEST;
        message = e.msg.as_str();
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else if let Some(e) = err.find::<OtherError>() {
        log::error!("{}", e.msg);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    } else {
        log::error!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
