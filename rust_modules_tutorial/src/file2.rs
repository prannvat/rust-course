//This file is a crate
mod person{
    pub struct personal_info{
        pub age:u8,
        pub education: String,
    }
    impl personal_info {
        pub fn new(new_edu: &str) -> Self{
            Self { age: 20, education: String::from(new_edu) }
        }
    }
}


pub fn some_person(){
    let mut person1 = person::personal_info::new("Bachelor honours");

}