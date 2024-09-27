mod interface_adaptors {
    pub mod handler;
}
mod application_business_rules {
    pub mod usecase;
}

use std::sync::Arc;

use application_business_rules::usecase::student::StudentUsecase;
use interface_adaptors::handler::student::StudentHandler;

use axum::{
    routing::{get, post},
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let usecase = StudentUsecase {
        id: 1,
        name: "John Doe".to_string(),
    };
    let handler = Arc::new(StudentHandler::new(usecase));

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(handler.create_student));

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Server listening on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
