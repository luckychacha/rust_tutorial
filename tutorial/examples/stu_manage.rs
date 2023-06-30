use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
struct Student {
    id: u32,
    name: String,
    age: u8,
    class_id: Option<u32>,
}
#[derive(Debug, Clone)]
struct Class {
    id: u32,
    name: String,
    students: Vec<Student>,
}
#[derive(Debug)]
struct Course {
    id: u32,
    name: String,
    class_id: u32,
}

#[derive(Debug)]
struct School {
    classes: Vec<Class>,
    courses: Vec<Course>,
}

impl School {
    fn new() -> School {
        School {
            classes: Vec::new(),
            courses: Vec::new(),
        }
    }

    fn add_class(&mut self, class: Class) {
        self.classes.push(class);
    }

    fn add_course(&mut self, course: Course) {
        self.courses.push(course);
    }

    fn find_class_by_id(&mut self, class_id: u32) -> Option<&mut Class> {
        for class in &mut self.classes {
            if class.id == class_id {
                return Some(class);
            }
        }
        None
    }

    fn find_course_by_id(&self, course_id: u32) -> Option<&Course> {
        self.courses.iter().find(|c| c.id == course_id)
    }

    fn add_student(&mut self, class_id: u32, student: Student) {
        if let Some(class) = self.find_class_by_id(class_id) {
            class.students.push(student);
        } else {
            println!("Class not found!");
        }
    }

    fn remove_student(&mut self, student_id: u32) {
        for class in &mut self.classes {
            if let Some(index) = class.students.iter().position(|s| s.id == student_id) {
                class.students.remove(index);
                return;
            }
        }
        println!("Student not found!");
    }
}

fn main() {
    let mut school = School::new();

    let mut class1 = Class {
        id: 1,
        name: "Class A".to_string(),
        students: Vec::new(),
    };

    let mut class2 = Class {
        id: 2,
        name: "Class B".to_string(),
        students: Vec::new(),
    };

    let course1 = Course {
        id: 1,
        name: "Math".to_string(),
        class_id: 1,
    };

    let course2 = Course {
        id: 2,
        name: "English".to_string(),
        class_id: 2,
    };

    school.add_class(class1.clone());
    school.add_class(class2.clone());

    school.add_course(course1);
    school.add_course(course2);

    let student1 = Student {
        id: 1,
        name: "John".to_string(),
        age: 18,
        class_id: None,
    };

    let student2 = Student {
        id: 2,
        name: "Alice".to_string(),
        age: 17,
        class_id: None,
    };

    school.add_student(class1.id, student1);
    school.add_student(class2.id, student2);

    println!("{:#?}", school);

    school.remove_student(2);

    println!("{:#?}", school);
}
