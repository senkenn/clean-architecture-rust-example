use crate::enterprise_business_rules::domain::entity;
use crate::enterprise_business_rules::domain::repository_interface::student::IStudentRepository;

pub struct InMemoryStudentRepository;

impl InMemoryStudentRepository {
    pub fn new() -> Self {
        InMemoryStudentRepository
    }
}

impl IStudentRepository for InMemoryStudentRepository {
    fn save(&self, student: entity::student::Student) {
        println!("Student saved: {}", student.name);
        // TODO: Implement the actual save logic
    }
}
