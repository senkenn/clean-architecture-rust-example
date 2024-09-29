// src/interface_adaptors/handler/student.rs
use std::sync::Arc;

use crate::application_business_rules::usecase::student::StudentUsecase;
use axum::{http::StatusCode, Json};
use serde::Deserialize;

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub id: u64,
    pub name: String,
}

pub struct StudentHandler {
    pub usecase: Arc<StudentUsecase>,
}

impl StudentHandler {
    pub fn new(usecase: Arc<StudentUsecase>) -> Self {
        StudentHandler { usecase }
    }

    pub async fn create_student(
        // Use Arc<Self> to allow shared ownership
        self: Arc<Self>,

        // this argument tells axum to parse the request body
        // as JSON into a `CreateUser` type
        Json(payload): Json<CreateUser>,
    ) -> StatusCode {
        let student = crate::enterprise_business_rules::domain::entity::student::Student {
            id: payload.id,
            name: payload.name,
        };
        self.usecase.clone().create_student(student).unwrap();

        // this will be converted into a JSON response
        // with a status code of `201 Created`
        StatusCode::CREATED
    }
}
