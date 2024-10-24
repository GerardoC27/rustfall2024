#[derive(Debug,Partial)]
pub enum Major {
    ComputerScience,
    ElectricalEngineering,
Undefined,
}


impl Major {
    pub fn classify(major:&str) -> Self {

    
    match major{
        "CS" => Major::CS,
        "EE" => Major::EE,
        _ => Major::Undefined,
    }
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_undefined_creating() {
        let s = Student::new("AAA", "Chemistry");
        assert_eq!(s.major,Major::Undefined);
    }
}