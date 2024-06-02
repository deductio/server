use rocket::response::Debug;

#[derive(Debug, Responder)]
pub enum DeductError {
    ReqwestError(Debug<reqwest::Error>),
    DieselError(Debug<diesel::result::Error>),

    #[response(status = 401)]
    UnauthorizedUser(&'static str),
    
    #[response(status = 400)]
    BadRequest(&'static str),

    #[response(status = 404)]
    NotFound(&'static str)
}

impl From<diesel::result::Error> for DeductError {
    fn from(err: diesel::result::Error) -> DeductError {
        match err {
            diesel::result::Error::NotFound => Self::NotFound("Not found"),
            _ => Self::DieselError(Debug(err))
        }
    }
}

impl From<reqwest::Error> for DeductError {
    fn from(err: reqwest::Error) -> DeductError {
        Self::ReqwestError(Debug(err))
    }
}

pub type DeductResult<T> = std::result::Result<T, DeductError>;
