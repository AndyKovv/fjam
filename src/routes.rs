/*
    Define application routes here
*/

use gotham::router::builder::*;
use gotham::router::Router;
use user::new_user_handler::NewUserHander;

/// Common method for routes
/// Think need implement single reponsibility router 
/// Next step should be implementation of error middleware for catch errors
/// and return it to client side.
pub fn router() -> Router {
    // Build the router
    build_simple_router(|router| {
        router.scope("/api", |route| {
            route.post("/new-user").to_new_handler(NewUserHander::new());
        });
    })
}