use std::sync::Arc;

use crate::enterprise_business_rules::domain::entity;
use crate::enterprise_business_rules::domain::repository_interface::student::IStudentRepository;

pub struct StudentUsecase {
    pub repo: Arc<dyn IStudentRepository>,
}

impl StudentUsecase {
    pub fn new(repo: Arc<dyn IStudentRepository>) -> Self {
        StudentUsecase { repo }
    }

    pub fn create_student(&self, id: u64, name: String) {
        println!("Student created: {}", name.clone());
        let student = entity::student::Student { id, name };
        self.repo.save(student);
    }
}
