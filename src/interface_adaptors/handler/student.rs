// mod application_business_rules;

// use crate::application_business_rules::usecase;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

// pub struct StudentHandler<'a> {
//     student_usecase: usecase::StudentUsecase<'a>,
// }

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

pub(crate) async fn create_student(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
