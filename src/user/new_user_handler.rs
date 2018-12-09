/*
    Handler for create new user directly from api.
    New user handle need to hold all create logic.
*/
use futures::{future, Future, Stream};
use gotham::error::Result;
use gotham::handler::{Handler, HandlerFuture, NewHandler, IntoHandlerError};
use gotham::helpers::http::response::create_empty_response;
use gotham::state::State; 
use gotham::state::FromState;

use serde::Serialize;
use hyper::{Body, StatusCode};
use user::dto::{CreateUserDTO};
use convert::ToMessagePack;
use rmps::{Deserializer, Serializer};

#[derive(Debug, Clone)]
pub struct NewUserHander;

impl NewUserHander {
    pub fn new() -> Self {
        Self{}
    }
}

impl Handler for NewUserHander {
    fn handle(self, mut state: State) -> Box<HandlerFuture> {
        let r = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(_valid_body) => {
                let res = create_empty_response(&state, StatusCode::OK);
                future::ok((state, res))
            }
            Err(err) => future::err((state, err.into_handler_error()))
        });
        Box::new(r)
    }
}

impl NewHandler for NewUserHander {
    type Instance = Self;

    fn new_handler(&self) -> Result<Self::Instance> {
        Ok(self.clone())
    }
}

#[cfg(test)]
pub mod tests_new_user_api {
    use super::*;
    use test_utils::server_utils::run_test_server;

    fn default_user_fixture() -> CreateUserDTO {
        CreateUserDTO {
            email: "andy.kovv@gmail.com".to_string(),
            password: "top_secret_passw".to_string()
        }
    }

    #[test]
    fn test_should_add_new_user_via_api() {
        let server = run_test_server();
        let user_data = default_user_fixture();
        println!("{:?}", user_data.convert());
        // let tex = serde_json::to_string(&user_data).unwrap();
        // let mut buf = Vec::new();
        // tex.serialize(&mut Serializer::new(&mut buf)).unwrap();
        // // user_data.serialize(&mut Serializer::new(&mut buf)).unwrap();
        // buf
    }
}