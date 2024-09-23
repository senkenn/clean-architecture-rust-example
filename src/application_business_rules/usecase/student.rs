pub struct StudentUsecase {
    // Define the fields and methods for StudentUsecase
    pub id: u64,
}

impl<'a> StudentUsecase {
    pub fn new(field: &'a str) -> Self {
        StudentUsecase { field }
    }

    pub fn create_user(&self, name: String) -> User {
        User {
            id: 1, // In a real application, this would be generated or retrieved.
            name,
        }
    }
}
