
use mime;
use gotham::handler::{Handler, NewHandler, HandlerFuture};
use gotham::helpers::http::response::create_response;
use gotham::state::State;
use futures::future;
use hyper::StatusCode;

use user::core::UserProfile;

impl Handler for UserProfile {
    fn handle(self, state: State) -> Box<HandlerFuture> {
        let res = {
            create_response(
                &state,
                StatusCode::OK,
                mime::TEXT_PLAIN,
                "some".to_string().into_bytes(),
            )
        };
        Box::new(future::ok((state, res)))
    }
}

impl NewHandler for UserProfile {
    type Instance = Self;
    fn new_handler(&self) -> std::result::Result<Self::Instance, gotham::error::Error> {
        Ok(self.clone())
    }
}