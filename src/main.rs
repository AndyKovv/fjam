#[macro_use]
extern crate log;

extern crate hyper;
extern crate diesel;
extern crate gotham;
extern crate fjam;

use std::sync::Once;

use diesel::pg::PgConnection;
use hyper::StatusCode;
use gotham::router::Router;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::new_pipeline;
use gotham::router::builder::*;
// use fjam::user::route::build_user_routes;

use fjam::user::core::{UserIdExtractor, UserProfile};
use fjam::middlewares::{create_postgres_middleware, DieselMiddleware};
use fjam::config::Config;


/// Create application router
fn router(middleware: DieselMiddleware<PgConnection>) -> Router {
    // add middleware to a new pipeline
    let (chain, pipelines) = single_pipeline(
        new_pipeline().add(middleware).build()
    );

    // Build the router
    build_router(chain, pipelines, |route| {
            route.get("/:id:[0-9]+")
            .with_path_extractor::<UserIdExtractor>()
            .to_new_handler(UserProfile::new());
    })
}

fn main() {
    // let connection = PG_POOL.get();
    // assert!(connection.is_ok());
    // create_postgres_middleware(&url);
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;

    #[test]
    fn test_should_check_builder() {
        let url = Config::get_postgress_connection_url();
        let middleware = create_postgres_middleware(&url);
        let test_server = TestServer::new(router(middleware)).unwrap();
        let response = test_server.client().get("http://localhost/1234321323").perform().unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        // let second = test_server.client().get("http://localhost/user/id/112321312").perform().unwrap();
        // assert_eq!(second.status(), StatusCode::Ok);
    }
}
