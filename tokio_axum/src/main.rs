pub mod error;
mod get;

use axum::Router;
use error::Result;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
	let subscriber = tracing_subscriber::fmt()
		.compact()
		.with_file(true)
		.with_line_number(true)
		.finish();

	tracing::subscriber::set_global_default(subscriber)?;

	let app = Router::new().nest("/api", get::router());

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	tracing::info!("Listening on {}", addr);

	let listener = tokio::net::TcpListener::bind(addr).await?;
	axum::serve(listener, app).await?;

	Ok(())
}
