
use diesel::pg::PgConnection;
use middlewares::DieselMiddleware;

use gotham::router::builder::{RouterBuilder, DrawRoutes, DefineSingleRoute};
use user::core::{UserIdExtractor, UserProfile};
// use borrow_bag::Take;
use borrow_bag::Handle;
use gotham::pipeline::chain::PipelineHandleChain;
// use borrow_bag::handle::Take;
use gotham::pipeline::Pipeline;


// pub fn build_user_routes(route: &mut RouterBuilder<PipelineHandleChain<DieselMiddleware<PgConnection>>, ()>
// ) {
//     // Function create user routes
//     // Add user route
//     route.get("/:id:[0-9]+")
//     .with_path_extractor::<UserIdExtractor>()
//     .to_new_handler(UserProfile::new());
// }
