extern crate hyper;
extern crate gotham;
extern crate fjam;

use hyper::StatusCode;
use gotham::router::Router;
use gotham::router::builder::*;
use fjam::user::route::build_user_routes;

fn router() -> Router {
    build_simple_router(|mut router|{
        build_user_routes(&mut router);
    })
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;

    #[test]
    fn test_should_check_builder() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server.client().get("http://localhost/1234321323").perform().unwrap();
        assert_eq!(response.status(), StatusCode::Ok);
        let second = test_server.client().get("http://localhost/user/id/112321312").perform().unwrap();
        assert_eq!(second.status(), StatusCode::Ok);
    }
}
