use crate::enterprise_business_rules::domain::entity;

impl entity::student::Student {
    pub fn is_valid(&self) -> bool {
        println!("id: {}, name: {}", self.id, self.name);
        true
    }
}
