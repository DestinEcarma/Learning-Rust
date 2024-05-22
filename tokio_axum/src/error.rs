use axum::{http::StatusCode, response::IntoResponse};
use derive_more::From;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
	#[from]
	Io(std::io::Error),
	#[from]
	SetGlobalDefaultError(tracing::subscriber::SetGlobalDefaultError),
}

impl IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		match &self {
			Error::Io(err) => {
				tracing::error!("{err}");

				(self.status_code(), "INTERNAL_SERVER_ERROR").into_response()
			}
			Error::SetGlobalDefaultError(err) => {
				tracing::error!("{err}");

				(self.status_code(), "INTERNAL_SERVER_ERROR").into_response()
			}
		}
	}
}

impl Error {
	pub fn status_code(&self) -> StatusCode {
		match self {
			_ => StatusCode::INTERNAL_SERVER_ERROR,
		}
	}
}
