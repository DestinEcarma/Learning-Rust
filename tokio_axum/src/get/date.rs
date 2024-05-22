use crate::error::Result;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn handler() -> Result<impl IntoResponse> {
	tracing::info!("GET /date");

	Ok((
		StatusCode::OK,
		Json(json!({ "date": "Thu May 23 2024 01:56:00 GMT+0800 (Philippine Standard Time)" })),
	))
}
