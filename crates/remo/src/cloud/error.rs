use failure::Fail;

#[derive(Debug, Fail)]
pub enum APIError {
    #[fail(display = "Error: {:?}", error)]
    Error { error: String },

    #[fail(display = "Authorization failed")]
    AuthError,

    #[fail(display = "Client error: {:?}", error)]
    HTTPError { error: reqwest::Error },
}

impl From<reqwest::Error> for APIError {
    fn from(error: reqwest::Error) -> Self {
        APIError::HTTPError { error }
    }
}
