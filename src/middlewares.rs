use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process;
use std::io;
use futures::{future, Future};

// use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::Connection;

use r2d2_diesel::ConnectionManager;
use r2d2::{Pool, PooledConnection, Error};
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::{request_id, State};
use gotham::handler::HandlerFuture;


/// Provides access to a Diesel connection wothin an r2d2 pool via Gotham State.
#[derive(StateData)]
pub struct Diesel<T> where T: Connection + 'static {
    pool: Pool<ConnectionManager<T>>,
}

impl<T> Diesel<T>
    where T: Connection + 'static, {
        pub(crate) fn new(pool: Pool<ConnectionManager<T>>) -> Self {
            Diesel { pool }
        }

        // Provides acces to a Diesel connection from r2d2 backend connection pool.
        pub fn conn(&self) -> Result<PooledConnection<ConnectionManager<T>>, Error> {
            self.pool.get()
        }
    }


/// Gotham conpatible middleware that manage a pool of Diesel connections via r2d2
pub struct DieselMiddleware<T> where T: Connection + 'static, {
    pool: AssertUnwindSafe<Pool<ConnectionManager<T>>>,
}

impl<T> DieselMiddleware<T> where T: Connection, {
    pub fn with_pool(pool: Pool<ConnectionManager<T>>) -> Self {
        DieselMiddleware {
            pool: AssertUnwindSafe(pool),
        }
    }
}


/// Instance created by DieselMiddleware for each
pub struct DieselMiddlewareImpl<T>
    where
    T: Connection + 'static,
{
    pool: Pool<ConnectionManager<T>>,
}

impl<T> NewMiddleware for DieselMiddleware<T>
    where
    T: Connection + 'static,
{
    type Instance = DieselMiddlewareImpl<T>;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        match catch_unwind(|| self.pool.clone()) {
            Ok(pool) => Ok(DieselMiddlewareImpl { pool }),
            Err(_) => {
                error!(
                    "PANIC: r2d2::Pool::clone caused a panic, unable to rescue with a HTTP error"
                );
                eprintln!(
                    "PANIC: r2d2::Pool::clone caused a panic, unable to rescue with a HTTP error"
                );
                process::abort()
            }
        }
    }
}

impl<T> Clone for DieselMiddleware<T>
where
    T: Connection + 'static,
{
    fn clone(&self) -> Self {
        match catch_unwind(|| self.pool.clone()) {
            Ok(pool) => DieselMiddleware {
                pool: AssertUnwindSafe(pool),
            },
            Err(_) => {
                eprintln!("PANIC: r2d2::Pool::clone caused a panic");
                process::abort()
            }
        }
    }
}

impl<T> Middleware for DieselMiddlewareImpl<T>
    where T: Connection + 'static {

        fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
            where Chain: FnOnce(State) -> Box<HandlerFuture> {
                trace!("[{}] pre chain", request_id(&state));
                state.put(Diesel::<T>::new(self.pool));
                let fut = chain(state).and_then(move |(state, response)| {
                    {
                        trace!("[{}] post chain", request_id(&state));
                    }
                    future::ok((state, response))
                });
                Box::new(fut)
        }
}

/// Create postgres middleware
/// url - connection url for postgres
pub fn create_postgres_middleware(url: &str) -> DieselMiddleware<PgConnection> {
        // Method should create middleware
        let manager = ConnectionManager::new(url);
        let pool = Pool::<ConnectionManager<PgConnection>>::new(manager).unwrap();
        DieselMiddleware::with_pool(pool)
}
