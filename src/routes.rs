/*
    Define application routes here
*/

use gotham::router::builder::*;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::Router;
use user::new_user_handler::NewUserHander;
use auth_middleware::AuthMiddleware;

/// Common method for routes
/// Think need implement single reponsibility router 
/// Next step should be implementation of error middleware for catch errors
/// and return it to client side.
pub fn router() -> Router {
    // Build the router

    let (chain, pipelines) = single_pipeline(new_pipeline().add(AuthMiddleware).build());

    build_router(chain, pipelines, |router| {
        router.scope("/api", |route| {
            route.post("/new-user").to_new_handler(NewUserHander::new());
        });
    })
}