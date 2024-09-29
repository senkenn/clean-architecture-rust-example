use crate::enterprise_business_rules::domain::entity;

impl entity::student::Student {
    pub fn is_valid(&self) -> bool {
        self.id > 0 && self.name.len() > 0
    }
}
