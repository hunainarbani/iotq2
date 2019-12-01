#[derive(Debug)]

struct Student{
    name: String,
    age: u8,
    address: String
}

impl Student {
    pub fn create_student(name: String, age: u8, address: String) -> Student {

        let newstudent = Student {
            name: name,
            age: age,
            address: address
        };

        newstudent
    }

    fn view_student(&self) -> String {

        let info_student = format!("Name: {}\nAge: {}\nAddress: {}",
                                    self.name,
                                    self.age,
                                    self.address);

        info_student                                    
    }
}

fn main() {
    
    let student_01 = Student::create_student("Hunain Arbani".to_string(),
                                             32,"Bahria Auditorium".to_string());

    
    println!("{}",student_01.view_student());

}
