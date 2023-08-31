//This file is a crate

pub fn printing(){

    println!("This is a library module for file 1");
}


mod maths{
    pub mod basic_math{
        pub fn multiplication(num1: &i32,num2: &i32) -> i32{
            num1*num2

        }
    }
}

pub fn rect_area (length: &i32, width: &i32) -> i32{
    use maths::basic_math::multiplication;
    multiplication(length, width)
}

/*
file1
        - maths
            - basic_math
                -multiplication()
        -rect_area()

*/
