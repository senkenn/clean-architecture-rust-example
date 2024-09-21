// mod controllers;
// mod entities;
mod domain;
// mod gateways;
// mod infrastructure;
mod adapters;
mod usecases;

use infrastructure::in_memory_user_repository::InMemoryUserRepository;
use controllers::user_controller::UserController;

fn main() {
    let repository = InMemoryUserRepository::new();
    let user_controller = UserController::new(&repository);

    match user_controller.create_user(1, String::from("John Doe"), String::from("john@example.com")) {
        Ok(user) => println!("User created: {:?}", user),
        Err(err) => println!("Failed to create user: {}", err),
    }
}
