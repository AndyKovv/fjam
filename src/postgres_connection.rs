/// Module for create connection to postgres
/// Add here new features of postres connections only

use std::borrow::Borrow;
use r2d2;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

use config::Config;

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;


fn pg_pool(url: &str) -> PgPool {
    // Function should create pool and return it connection
    let manager = ConnectionManager::<PgConnection>::new(url);
    // Think need panic thread if connection failed
    r2d2::Pool::builder().build(manager).expect("Failed to create pool")
}

lazy_static! {

    /// Use PG_POOL for use connection pool
    static ref PG_POOL: PgPool = {
        let url = Config::get_postgress_connection_url();
        pg_pool(&url)
    };
}

pub fn pool<'a>() -> &'a PgPool {
    // Function should return posgres connection pool
    &PG_POOL
}

// pub fn connection<'a>() -> &'a PgConnection {
//         &*PG_POOL.get().unwrap()
//     // &pool.try_get().unwrap()
//     // match c.try_get() {
//     //     Some(data) => *data,
//     //     None => panic!("{:?}", "Some"),
//     // }
    
//     // match *PG_POOL.try_get() {
//     //     Some(p) => p,
//     //     None => panic!("{:?}", "some")
//     // }
// }

