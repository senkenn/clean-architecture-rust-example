use crate::application_business_rules::usecase::student::StudentUsecase;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    id: u64,
    name: String,
}

pub struct StudentHandler {
    pub usecase: StudentUsecase,
}

impl StudentHandler {
    pub fn new(usecase: StudentUsecase) -> Self {
        StudentHandler { usecase }
    }

    pub(crate) async fn create_student(
        // this argument tells axum to parse the request body
        // as JSON into a `CreateUser` type
        Json(payload): Json<CreateUser>,
    ) -> StatusCode {
        self.usecase.create_student(payload.id, payload.name);

        // this will be converted into a JSON response
        // with a status code of `201 Created`
        StatusCode::CREATED
    }
}
