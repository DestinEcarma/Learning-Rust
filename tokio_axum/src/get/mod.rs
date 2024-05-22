mod date;

use axum::{routing, Router};

pub fn router() -> Router {
	Router::new().route("/date", routing::get(date::handler))
}
