use crate::handlers::population::{
    all_population, latest_population, latest_population_total, save_all_population,
};
use crate::handlers::users::list_users;
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_router() -> Router {
    Router::new()
        .route("/users", get(list_users))
        .route("/population", get(all_population))
        .route("/population", post(save_all_population))
        .route("/population/{country_code}", get(latest_population))
        .route(
            "/population/{country_code}/total",
            get(latest_population_total),
        )
}
