/*
    Handler for create new user directly from api.
    New user handle need to hold all create logic.
*/
use futures::{future, Future, Stream};
use gotham::error::Result;
use gotham::handler::{Handler, HandlerFuture, NewHandler, IntoHandlerError, HandlerError};
use gotham::helpers::http::response::create_empty_response;
use gotham::state::State; 
use gotham::state::FromState;
use hyper::{Body, StatusCode};
use user::dto::{CreateUserDTO};

use common::common_api_errors::{Errors, BasicApiError};
use convert::{ToMessagePack};
use convert::errors::CommonConvertError;
use common::common_api_errors::HasStatusCode;

impl HasStatusCode for CommonConvertError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}




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
            Ok(valid_body) => {
                let body = &valid_body.into_bytes().as_ref().to_vec();
                let user_dto = CreateUserDTO::from_message_pack(body);
                match user_dto {
                    Ok(_data) => {
                        let res = create_empty_response(&state, StatusCode::OK);
                        future::ok((state, res))
                    },
                    Err(err) => {
                        let error: BasicApiError = err.into();
                        error.into_future_response(state)
                    }
                }
            }
            Err(_) => {
                let res = create_empty_response(&state, StatusCode::OK);
                future::ok((state, res))
            }
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
    use mime;
    use test_utils::server_utils::{run_test_server, get_url};
    use convert::ToMessagePack;
    fn default_user_fixture() -> CreateUserDTO {
        CreateUserDTO {
            email: "andy.kovv@gmail.com".to_string(),
            password: "top_secret_passw".to_string()
        }
    }


    #[test]
    fn test_should_raise_error_with_invalid_data_income() {
        let api_url = "/api/new-user";
        let url = get_url(api_url);
        let payload = &[182, 123, 34, 115, 101, 99, 111, 110, 100, 95, 110, 97, 109, 101, 34, 58, 34, 65, 110, 100, 121, 34, 125];

        let response = run_test_server()
            .client()
            .post(url, String::from_utf8_lossy(payload), mime::TEXT_PLAIN)
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = response.read_body().unwrap();
        assert_eq!(body, [217, 34, 123, 34, 101, 114, 114, 111, 114, 34, 58, 34, 69, 114, 114, 111, 114, 32, 100, 101, 115, 101, 114, 105, 97, 108, 105, 122, 101, 32, 100, 97, 116, 97, 34, 125].to_vec());
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