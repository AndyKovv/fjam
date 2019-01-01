
extern crate mime;
use futures::future::FutureResult;
use futures::future;
use gotham::helpers::http::response::create_response;

use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use hyper::{StatusCode, Response, Body};
use gotham::state::State;
use gotham::handler::HandlerError;
use convert::errors::CommonConvertError;
use common::error_dto::ErrorDTO;


#[derive(Debug)]
pub enum Errors {
    ApiError(Box<BasicApiError>)
}

pub trait HasStatusCode: Error {
    fn status_code(&self) -> StatusCode;
}

impl  Display for Errors {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Errors::ApiError(ref err) => err.fmt(f),
        }
    }
}


impl Error for Errors {
    fn description(&self) -> &str {
        match *self {
            Errors::ApiError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            Errors::ApiError(ref err) => Some(err),
        }
    }
}

impl From<Box<BasicApiError>> for Errors {
    fn from(err: Box<BasicApiError>) -> Self {
        Errors::ApiError(err)
    }
}

#[derive(Debug, Clone)]
pub struct BasicApiError {
    details: String,
    status_code: StatusCode,
}

impl BasicApiError {
    pub fn new<T: HasStatusCode>(handler: T) -> Box<Self> {
        Box::new(Self {
            details: handler.description().to_string(),
            status_code: handler.status_code(),
        })
    }
    
    pub fn into_future_response(&self, state: State) -> FutureResult<(State, Response<Body>), (State, HandlerError)> {
        /* Method should covert error with future response */
        let payload = ErrorDTO::to_mspack(&self.details);
        let res = create_response(
            &state, self.status_code,
            mime::APPLICATION_MSGPACK,
            payload
        );
        future::ok((state, res))
    }
}

impl From<CommonConvertError> for BasicApiError {
    fn from(err: CommonConvertError) -> Self {
        Self {
            details: err.description().to_string(),
            status_code: StatusCode::BAD_REQUEST,
        }
    }
}

impl  Display for BasicApiError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.details)
    }
}

impl  Error for BasicApiError {
    fn description(&self) -> &str {
        &self.details
    }
}