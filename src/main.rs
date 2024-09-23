// // mod entities;
// mod domain;
// // mod gateways;
// // mod infrastructure;
// mod adapters;
// mod usecases;

// use infrastructure::in_memory_user_repository::InMemoryUserRepository;
// use controllers::user_controller::UserController;

// fn main() {
//     let repository = InMemoryUserRepository::new();
//     let user_controller = UserController::new(&repository);

//     match user_controller.create_user(1, String::from("John Doe"), String::from("john@example.com")) {
//         Ok(user) => println!("User created: {:?}", user),
//         Err(err) => println!("Failed to create user: {}", err),
//     }
// }

mod application_business_rules;

mod interface_adaptors {
    pub mod handler;
}

use interface_adaptors::handler::student::create_student;

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

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_student));

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Server listening on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
