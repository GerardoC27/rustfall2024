
use crate::major::Major;
#[derive(Debug)]
pub struct Student {
    name:String,
    major:Major,
}

impl Student {
    pub fn new() -> Student {
        Student {
            name:name.to_string(),
            major:Major::classify(major),
        }
    }
}