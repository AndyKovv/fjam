
use std::io;
use mime;
use gotham::handler::{Handler, NewHandler, HandlerFuture};
use gotham::http::response::create_response;
use gotham::state::State;
use futures::future;
use hyper::StatusCode;

use user::core::UserProfile;

impl Handler for UserProfile {
    fn handle(self, state: State) -> Box<HandlerFuture> {
        let res = {
            create_response(
                &state,
                StatusCode::Ok,
                Some(("some".to_string().into_bytes(), mime::TEXT_PLAIN)),
            )
        };
        Box::new(future::ok((state, res)))
    }
}

impl NewHandler for UserProfile {
    type Instance = Self;
    fn new_handler(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}