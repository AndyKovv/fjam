/*
    This file is for create server test utils for server side.
    Wheen need to add some test utils for server add this here.
*/
use gotham::test::TestServer;
use routes::router;


/// Get server function returns test server with.
/// By default this get rouute from system.
pub fn run_test_server() -> TestServer {
    TestServer::new(router()).unwrap()
}

/// Create url for make request to test server
pub fn get_url(url: &str) -> String {
    let host = "http://localhost";
    format!("{}{}", host, url)
}