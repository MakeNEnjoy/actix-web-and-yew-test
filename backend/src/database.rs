use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Student {
    name: String,
    age: u8,
    grade: u8
}

#[derive(Debug)]
pub struct Database {
    students: Vec<Student>,
}

impl Database {
    pub fn new() -> Database {
        let students = vec![
            Student {
                name: "John".to_string(),
                age: 15,
                grade: 10
            },
            Student {
                name: "Jane".to_string(),
                age: 16,
                grade: 11
            },
            Student {
                name: "Jack".to_string(),
                age: 17,
                grade: 12
            },
            Student {
                name: "Jill".to_string(),
                age: 18,
                grade: 13
            },
            Student {
                name: "Jen".to_string(),
                age: 19,
                grade: 14
            },
        ];
        Database { students: students }

    }
    pub fn get_first_3_students(&self) -> Vec<Student> {
        self.students[0..3].to_vec()
    }
}