pub struct StudentUsecase {
    // Define the fields and methods for StudentUsecase
    pub id: u64,
    pub name: String,
}

impl StudentUsecase {
    pub fn create_student(&self, id: u64, name: String) {
        // User {
        //     id: 1, // In a real application, this would be generated or retrieved.
        //     name,
        // }
        println!("Student created: {}", self.name);
    }
}
