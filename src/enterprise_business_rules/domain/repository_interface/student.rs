use crate::enterprise_business_rules::domain::entity;

pub trait IStudentRepository: Send + Sync {
    fn save(&self, student: entity::student::Student);
}
