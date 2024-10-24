mod student;
mod major;

fn main() {
    let s = student::Student::new("John", "CS");
    println!("{:?}", s);

    
}