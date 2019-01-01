
// Middleware for authenticate user

use futures::{future, Future};

use gotham::state::{State};
use gotham::handler::HandlerFuture;
use gotham::middleware::{Middleware};
use hyper::{StatusCode};
use gotham::helpers::http::response::create_empty_response;


#[derive(Debug, Clone, Copy, NewMiddleware)]
pub struct AuthMiddleware;


/// Middleware uses for authenticate user
impl Middleware for AuthMiddleware {
    fn call<Chain>(self, state: State, chain: Chain) -> Box<HandlerFuture>
        where Chain: FnOnce(State) -> Box<HandlerFuture> + Send {
            let result = chain(state);

            let f = result.then(|x| {
                match x {
                    Ok((state, response)) => {
                        future::ok((state, response))
                    },
                    Err((state, _err)) => {
                        let res = create_empty_response(&state, StatusCode::OK);
                        future::ok((state, res))
                    }
                }
            });
            Box::new(f)
    }
}

