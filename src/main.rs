mod interface_adaptors {
    pub mod handler;
    pub mod repository;
}
mod application_business_rules {
    pub mod usecase;
}
mod enterprise_business_rules {
    pub mod domain {
        pub mod entity;
        pub mod model;
        pub mod repository_interface;
    }
}

use std::sync::Arc;

use application_business_rules::usecase::student::StudentUsecase;
use enterprise_business_rules::domain::repository_interface::student::IStudentRepository;
use interface_adaptors::{
    handler::student::{CreateUser, StudentHandler},
    repository,
};

use axum::{
    extract::{Json, State},
    http::StatusCode,
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

    let repository: Arc<dyn IStudentRepository> =
        Arc::new(repository::student::InMemoryStudentRepository::new());
    let usecase = Arc::new(StudentUsecase::new(repository));
    let handler = Arc::new(StudentHandler::new(usecase));

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(handle_create_student))
        .with_state(handler);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Server listening on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

// Define your handler function with #[axum::debug_handler]
#[axum::debug_handler]
async fn handle_create_student(
    State(handler): State<Arc<StudentHandler>>,
    Json(payload): Json<CreateUser>,
) -> StatusCode {
    handler.create_student(Json(payload)).await
}
