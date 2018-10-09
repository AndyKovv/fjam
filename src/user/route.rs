
use gotham::router::builder::{RouterBuilder, DrawRoutes, DefineSingleRoute};
use user::core::{UserIdExtractor, UserProfile};

pub fn build_user_routes(route: &mut RouterBuilder<(), ()>) {
    // Function create user routes
    // Add user route
    route.get("/:id:[0-9]+")
    .with_path_extractor::<UserIdExtractor>()
    .to_new_handler(UserProfile::new());
}
