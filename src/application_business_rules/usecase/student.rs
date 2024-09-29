use std::sync::Arc;

use crate::enterprise_business_rules::domain::entity::student::Student;
use crate::enterprise_business_rules::domain::repository_interface::student::IStudentRepository;

#[derive(Clone)]
pub struct StudentUsecase {
    pub repo: Arc<dyn IStudentRepository>,
}

impl StudentUsecase {
    pub fn new(repo: Arc<dyn IStudentRepository>) -> Self {
        StudentUsecase { repo }
    }

    pub fn create_student(&self, student: Student) -> Result<(), &'static str> {
        if student.is_valid() {
            self.repo.save(student);
            Ok(())
        } else {
            Err("Invalid student data")
        }
    }
}
