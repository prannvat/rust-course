use std::{sync::Arc, f32::consts::PI};

fn main() {
    println!("Hello world, from rust!");
    print!("Hello");
    println!(" The value is {}", 10);

    println!(
        "My first name is {} and my last name is {}",
        "Prannvat", "Singh"
    );
    /*
    multiline comment.
     */

    //Variables

    //Variables are immutable unless you specify using 'mut'
    let mut x = 15;
    x += 1;
    println!("The variable x is {x}");

    //Scalar data types
    println!("Max number in i8 is {}", std::i8::MAX);
    let status = false;

    println!("Values of variables are {:?}", (x, status));

    let (first_number, second_number) = (250, 480.22);
    let large_number = 1_000_000;
    println!("{first_number} and {second_number} and {large_number}");

    let n1 = 14;
    let n2 = 15.6;
    let n3 = n1 + n2 as i32;
    println!("{n3}");

    //Shadowing
    let s = 5;
    let s = 5 * 5;
    println!("{s}");

    //Constants
    //Not same as immutable variables, datatypes must be defined and cannot be inferred

    const MAX: i32 = 100000;
    println!("{MAX}");

    //Strings
    /*
    ~ Fixed length size (&str)
    ~ Growable strings (String)
     */

    let _some_string = "Fixed length string";
    let mut growable_string = String::from("This string will grow");
    println!("The string is \"{growable_string}\"");

    growable_string.push('!');
    println!("The string is \"{growable_string}\"");

    growable_string.pop();
    println!("The string is \"{growable_string}\"");

    growable_string.push_str(" which i will use");
    println!("The string is \"{growable_string}\"");

    //Functions on strings
    println!(
        "Basic functions on strings,
        is_empty(): {},
        len() : {},
        capacity() : {},
        contains(): {},",
        growable_string.is_empty(),
        growable_string.len(),
        growable_string.capacity(),
        growable_string.contains("use")
    );

    let num = 6;
    let num_string = num.to_string();
    println!("{num_string}");

    let empty_string = String::new();

    println!("{empty_string}");

    //Tuples
    let my_information = ("Salary", 40_000);
    println!("{} is equal to {}", my_information.0, my_information.1);

    let nested_tuple = (4, 5.0, (3, 2), "hello");
    let element = nested_tuple.2;
    println!("{}", element.0);

    //Arrays
    let mut number_array = [0, 1, 2, 3, 4, 5];
    println!("{} is not equal to {}", number_array[0], number_array[1]);
    println!("{:?}", number_array);

    number_array[4] = 5;

    let _array_with_same_elements = [0; 10];
    let mut string_array = ["apples", "orange", "banana"];
    let _string_array_2 = ["Unknown"; 6];
    string_array[0] = "mango";

    let _array_of_chars = ['a', 'b', 'c', 'd'];

    let subset_array = &number_array[0..3];
    println!("{:?}", subset_array);
    //& in this case is a pointer to memory location of array.

    println!("Elements in array are {}", number_array.len());
    println!(
        "The array is occupying {} bytes",
        std::mem::size_of_val(&number_array)
    );

    let check_index: Option<&i32> = number_array.get(100);
    println!("{:?}", check_index);

    //-------------------------------------------------------------------------------
    //ASSIGNMENT
    println!(
        "{}",
        distance_between_points_using_tuples((40, 2), (36, 17))
    );
    println!(
        "{}",
        distance_between_points_using_arrays([40.0, 2.0], [36.0, 17.0])
    );
    println!(
        "{}",
        euclidean_distance_between_points_using_tuples((0.0, 2.0), (0.0, 0.0))
    );

    //Selection
    let new_int = 5;

    if new_int == 2 {
        println!("New integer is 2")
    } else if new_int == 4 {
        println!("New integer is 4")
    } else {
        println!("New integer is not 2 or 4")
    }
    //Iteration
    for element in subset_array {
        print!("{} ", element);
    }

    //Vectors
    let mut num_vectors = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{} ", num_vectors[0]);
    println!("{:?} ", num_vectors);
    num_vectors[0] = 5;

    let array_with_same_element = vec![0; 10];
    let mut string_array = vec!["apple", "orange", "banana"];
    let string_array_2 = vec!["Unknown"; 6];

    string_array[0] = "papaya";

    let vector_char = vec!['a', 'b', 'c', 'd', 'e'];
    let subset_vec = &&num_vectors[0..3];
    let check_index = num_vectors.get(100);
    println!("{:?}", check_index);
    num_vectors.push(30);
    num_vectors.push(40);
    println!("the values are {:?}", num_vectors);

    num_vectors.remove(5);

    println!("The value of 10 exists: {}", num_vectors.contains(&10));

    //User inputs
    // let mut n = String::new();
    // println!("What is your favourite float number?");
    // std::io::stdin()
    //     .read_line(&mut n)
    //     .expect("failed to read input");

    // let n: f64 = n.trim().parse().expect("invalid input");

    let orig = 4;
    println!("{}", orig);
    println!("{}", add_three(orig));
    println!("{}", orig);

    // primitive_and_non_primitive_types();

    // control_structures();

    // breaks_continues();

    // square_of_sums_and_sum_of_squares();
    
    // sum_of_multiples_of_three_and_five();

    // total_production();
    // cars_produced_per_minute();

    // palindrome();

    // circle_rectangle();
    
    // enums();

    // point_struct();

    // option_enum();
    // hash_maps();
//    output_above();

//    run_student_assignment();

    // lifetimes();
    // closures();
    // fuction_types();

    // iterators();
    assignment_ch_five();

}

fn distance_between_points_using_tuples(p1: (i32, i32), p2: (i32, i32)) -> String {
    let x_difference: i32 = (p1.0 - p2.0).abs();

    let y_difference: i32 = (p1.1 - p2.1).abs();

    let difference_tuple = (x_difference, y_difference);
    return format!("Vector value of p1 to p2 is {:?}", difference_tuple);
}

fn distance_between_points_using_arrays(p1: [f64; 2], p2: [f64; 2]) -> String {
    let x_difference: f64 = (p1[0] - p2[0]).abs();

    let y_difference: f64 = (p1[1] - p2[0]).abs();

    let difference_array = (x_difference, y_difference);
    return format!("Vector value of p1 to p2 is {:?}", difference_array);
}

fn euclidean_distance_between_points_using_tuples(p1: (f64, f64), p2: (f64, f64)) -> String {
    let euclidean_distance = ((p1.0 - p2.0).powf(2.0) + (p1.1 - p2.1).powf(2.0)).sqrt();

    format!(
        "Euclidian distance between p1 and p2 is {:?}",
        euclidean_distance
    )
}

fn add_three(mut x: i32) -> i32 {
    x + 3
}

fn primitive_and_non_primitive_types() {
    //Rust Ownership
    /*
    ~ Each value in Rust has a variable that's called its owner
    ~ There can be only one owner at one time
    ~ When the owner goes out of scope, the value will be dropped
     */

    let x = 45.3;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s1 = String::from("abc");
    let s2 = &s1; // & used to reference value via memory location but s2 doesn't own it
    println!("s1 = {}, s2 = {}", s1, s2);

    //Primitive types: Can't be empty and are fixed size
    //Non-primitive types: Can be empty and can grow in size

    let vec_1 = vec![1, 2, 3, 4, 5];
    let vec_2 = &vec_1;
    let vec_3 = vec_2.clone(); //new copy of vec_2 (which is essentially making new copy of vec_1)

    println!(
        "vec_1 = {:?}, vec_2 = {:?}, vec_3 = {:?}",
        vec_1, vec_2, vec_3
    );

    {
        let my_name = String::from("Prannvat Singh");
    } //Code block thus variable memory in code block is dropped once out of scope

    let stack_num = 32;
    let mut heap_vec = vec![4, 5, 6];

    stack_function(stack_num);
    println!("{}", stack_num);

    heap_function(&mut heap_vec); //&mut means it is a mutable reference, so can be changed and is mutating original value in heap_vec

    /* Here we pass in a mutable reference to heap_vec. i.e. The ownership remains with heap_vec
    the function will get a reference to heap_vec which is mutable.

    If we pass in heap_vec without &, then the ownership will pass to the the variable defined
    inside the function. Hence, when the function finishes, the ownership will be out of scope,
    and the value will be dropped -> head_vec will no longer have a value.

    If we pass &heap_vec without the &mut then we cannot make changes to the value this reference
    inside the called function because this reference is not mutable.
    */

    println!("The value inside the main of heap_vec: {:?}", heap_vec);
}

fn stack_function(mut var: i32) {
    var = 56;
    println!("Var = {}", var);
}

fn heap_function(var: &mut Vec<i32>) {
    /*
    Here we receive a mutable reference which means ownership isnt passed to var.
    var is just a reference which is also mutable which can mutate the original
    value stored in the variable who's reference is passed in.
    */
    var.append(&mut vec![1, 2, 3, 4, 5, 6, 7, 8]);
    println!("Var: {:?}", var);
}

fn mutable_and_immutable_reference() {
    /*
    --------------------------------------------------------------------
    Reference rules
        ~ one mutable reference in a scope
        ~ Many immutable references
        ~ Mutable and immutable cannor coexist
        ~ Scope of a reference - scope starts where variable is defined and ends when variable is last used
        ~ Data should not change when immatable references are in scope
     */

    let mut heap_num = vec![4, 5, 6];
    let ref1: &mut Vec<i32> = &mut heap_num;
    // let ref2 = &mut heap_num;
    // println!("ref1: {:?} ref2: {:?}", ref1, ref2);
    //^^ Cannot do above becuase not allowed to have more than one mutable reference in a scope

    //Mutable and immutable cannot coexist in a scope

    let mut heap_num = vec![4, 5, 6];
    let ref1: &Vec<i32> = &heap_num; //scope of ref 1 starts here
    let ref2: &Vec<i32> = &heap_num;
    println!("ref1: {:?} ref2: {:?}", ref1, ref2); //scope of ref2 ends here
                                                   // scope starts where defined and ends when last used

    // Data should not change when immatable references are in scope
    let mut heap_num = vec![4, 5, 6];
    let ref1: &Vec<i32> = &heap_num;
    let ref2: &Vec<i32> = &heap_num;

    println!("ref1: {:?} ref2: {:?}", ref1, ref2);
    heap_num.push(68);
}

fn control_structures() {
    //CONDITIONAL STRUCTS
    let marks = 95;
    let grades = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else {
        'D'
    };

    //MATCH value (LIKE SWITCH CASE)
    let some_number = 1;
    match some_number {
        1 => println!("The number is 1"),
        2 | 3 => println!("The number is 2 or 3"),
        4..=100 => println!("The number is between 4 and 100"),
        _ => println!("The number is greater than 100"),
    }

    let marks = 50;
    let mut grade = 'N';
    match marks {
        90..=100 => grade = 'A',
        _ => grade = 'F',
    };

    // loop {
    //     println!("Hello, world!");
    // }

    let mut num = 2;
    while num < 4 {
        num += 1;
        println!("Less than 4");
    }

    let mut guess = false;
    println!("Guess my number between 1 and 5");
    while guess == false {
        let mut num = String::new();
        std::io::stdin()
            .read_line(&mut num)
            .expect("Error reading input");

        let num: u8 = num.trim().parse().expect("invalid input");
        if num == 5 {
            guess = true;
            println!("You got it!!");
        } else {
            println!("Try again");
        }
    }

    let mut some_vec = vec![45, 30, 85, 90, 41, 39];
    for i in some_vec.iter() {
        println!("The value in vector is {}", i);
    }
    for i in 0..some_vec.len() {
        println!("The {}th value in vector is {}", i + 1, some_vec[i]);
    }
    for i in some_vec.iter_mut() {
        *i += 1;
    }
}

fn breaks_continues() {
    let mut var = 100;
    for i in 0..100 {
        var -= 1;
        if var % 13 == 0 {
            println!("{var}");
            break;
        }
    }
}

fn square_of_sums_and_sum_of_squares() {

    println!("Enter amount of numbers you want to do func on: ");
    let mut userNumsLen = String::new();
    std::io::stdin()
    .read_line(&mut userNumsLen)
    .expect("Error reading input");

    let userNumsLen :i32 = userNumsLen.trim().parse().expect("Invalid input");

    let mut sumSquares = 0;
    for i in 0..= userNumsLen{
        sumSquares += i.pow(2);
    }

    let mut sumNums :i32 = 0;
    for i in 1..=userNumsLen{
        sumNums += i;
    }
    sumNums = sumNums.pow(2);

    println!("Difference between square of sum of first {} numbers and sum of squares of first {} is {}",userNumsLen,userNumsLen, sumNums - sumSquares);

     
}

fn sum_of_multiples_of_three_and_five(){
    println!("Enter amount of numbers you want to do func on: ");
    let mut user_nums_len = String::new();
    std::io::stdin()
    .read_line(&mut user_nums_len)
    .expect("Error reading input");

    let user_nums_len :i32 = user_nums_len.trim().parse().expect("Invalid input");

    let mut sumNums = 0;
    for i in 1..user_nums_len{
        if i % 3 == 0 || i % 5 == 0{
            sumNums += i;
        }      
    }
    println!("Sum of multiples of 3 and 5 upto {} equals: {}", user_nums_len, sumNums);
}

fn total_production(){
    println!("Enter number of hours to produce cars");
    let mut factory_hrs = String::new();
    std::io::stdin()
    .read_line(&mut factory_hrs)
    .expect("Error reading input");
    let factory_hrs : i32 = factory_hrs.trim().parse().expect("Invalid input");
    
    println!("Enter speed of assembly line");
    let mut assembly_speed = String::new();
    std::io::stdin()
    .read_line(&mut assembly_speed)
    .expect("Error reading input");
    let assembly_speed : i32 = assembly_speed.trim().parse().expect("Invalid input"); 
    
    let mut  prob = 0.0;
    
    match assembly_speed {
        1..=4 => prob = 1.0,
        5..=8 => prob = 0.9,
        9..=10 => prob = 0.77,
        _ => println!("invalid assembly speed")
    }  

    let cars_produced_in_hrs = (assembly_speed as f64*221 as f64 *prob* factory_hrs as f64) as i32;
    println!("Total number of cars produced in {} hrs is {} at given speed", factory_hrs, cars_produced_in_hrs);
    

}


fn cars_produced_per_minute() {
    println!("Enter number of hours to produce cars");
    let mut factory_hrs = String::new();
    std::io::stdin()
    .read_line(&mut factory_hrs)
    .expect("Error reading input");
    let factory_hrs : i32 = factory_hrs.trim().parse().expect("Invalid input");
    
    println!("Enter speed of assembly line");
    let mut assembly_speed = String::new();
    std::io::stdin()
    .read_line(&mut assembly_speed)
    .expect("Error reading input");
    let assembly_speed : i32 = assembly_speed.trim().parse().expect("Invalid input"); 
    
    let mut  prob = 0.0;
    
    match assembly_speed {
        1..=4 => prob = 1.0,
        5..=8 => prob = 0.9,
        9..=10 => prob = 0.77,
        _ => println!("invalid assembly speed")
    }  

    let cars_produced_in_hrs = (assembly_speed as f64*221 as f64 *prob* factory_hrs as f64) as i32;
    
     let cars_produced_in_mins = cars_produced_in_hrs/60/factory_hrs;
     println!("Cars per minute: {}", cars_produced_in_mins);
}


fn palindrome(){
    println!("Enter string to check if its a palindrome");
    let mut palindrome_inp = String::new();
    std::io::stdin()
    .read_line(&mut palindrome_inp)
    .expect("Error reading input");

    let palindrome_inp: &str = palindrome_inp.trim();
    let mut palindrome_vec : Vec<char> = vec![];
    for c in &mut palindrome_inp.chars(){
        palindrome_vec.push(c.to_ascii_lowercase());
    }

    let mut palindrome_check : Vec<char> = vec![];
    for element in &mut palindrome_inp.chars().rev(){
        palindrome_check.push(element.to_ascii_lowercase());
    }
    let mut valid = false;
    for i in 0..palindrome_inp.len(){
        if palindrome_vec[i] == palindrome_check[i]{
            valid = true;
        }
        else {
            valid = false;
        }
    }

    if valid == true{
        println!("Input {:?} IS a palindrome!", palindrome_inp)
    }
    else {
        println!("Input {:?} is NOT a palindrome!", palindrome_inp)
    };
    
}


struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32,

}

impl Person {

    //initialiser function -- constructor
    fn new() -> Self {
        Person { 
            citizenship: String::from("England"), 
            name: String::from("Bob"),
            age: 30, 
            gender: 'M', 
            salary: 20_000 }
    }

    fn structure_basics() {
    
        let person1 = Person{
            name: String::from("Prannvat Singh"),
            citizenship: String::from("British"),
            age: 18,
            gender: 'M',
            salary: 2_000

        };
        println!("The structure values are {} {} {}", person1.name, person1.citizenship, person1.age);
        println!("The taxes on Person {} is {}", person1.name, person1.compute_taxes());

        let mut person2 = Person::new();
        
        println!("The structure values are {} {} {}", person2.name, person2.citizenship, person2.age);
        println!("The taxes on Person {} is {}", person2.name, person2.compute_taxes());
        person2.age = 18;
        println!("The structure values are {} {} {}", person2.name, person2.citizenship, person2.age);

        let person3 = Person{
            age: 20,
            ..person1 //base construct
        };
        //initialising person 3, changing age and keeping rest attributes from person1
        println!("The structure values are {} {} {}", person3.name, person3.citizenship, person3.age);
        println!("The taxes on Person {} is {}", person3.name, person3.compute_taxes())
        
    }

    fn compute_taxes(&self) -> f32 {
        (self.salary as f32) * 0.2 

    }
}



/*
Simple tuples do not have an associated name but tuple structures do
 */

struct Numbers(i32, i32);

impl Numbers{

    fn tuple_structures(){
        let some_nums = Numbers(32,16);
        println!("The values of the two fields are {} and {}", some_nums.0, some_nums.1);

    }

    fn greater_than(&self) -> i32{
        if self.0 > self.1 {self.0} else {self.1}
    }
    fn lesser_than(&self) -> i32{
        if self.0 < self.1 {self.0} else {self.1}
    }
}

//--------------------------------------------------------------------------
// TRAITS AND DEFAULT IMPLEMENTATION
//--------------------------------------------------------------------------

struct Animal{  
    breed : String,
    colour: String,
    age: u8,
}

trait GeneralInfo{
    fn info(&self) -> (&str,&str,u8);
    fn breed_info(&self) -> &str;
}

impl GeneralInfo for Animal{
    fn info(&self) -> (&str,&str,u8){
        (&self.breed, &self.colour, self.age)
    }
    fn  breed_info(&self) -> &str {
        &self.breed
    }
}

struct  Dog{
    name: String,
    breed: String,
    colour: String,
    age: u8,

}


impl GeneralInfo for Dog{
    fn info(&self) -> (&str,&str,u8){
        (&self.name,&self.colour,self.age)
    }

    fn breed_info(&self) -> &str {
        &self.breed
    }
}


fn traits(){
    let best_boy = Dog{
        name: String::from("Hubble"),
        age: 2,
        colour: String::from("Golden"),
        breed: String::from("Golden Retriever"),
    };

    println!("{} is the best little {}", best_boy.name, best_boy.breed);

    println!("Basic info: {:?}", best_boy.info());
}


struct  Rectangle{
    length: f32,
    width: f32,

}

struct Circle{
    radius: f32,
}

trait ShapeInfo{
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

impl ShapeInfo for Circle{
    fn area(&self) -> f32 { 
        self.radius.powf(2.) * 3.14
        
        
    }
    fn perimeter(&self) -> f32 {
        self.radius*2.*3.14
    }

}


impl ShapeInfo for Rectangle{
    fn area(&self) -> f32 { 
        self.length*self.width
        
    }
    fn perimeter(&self) -> f32 {
        self.length*2. + self.width*2.
    }

}

struct Square{
    length_of_side: f32,
}

impl ShapeInfo for Square{
    fn area(&self) -> f32 {
       self.length_of_side.powf(2.)
    }
    fn perimeter(&self) -> f32 {
        self.length_of_side*4.
    }
}


fn circle_rectangle(){
    let circle1 = Circle{
        radius: 2.,
    };

    let rectangle1 = Rectangle{
        length: 5.,
        width: 2.5,
    };

    let square1 = Square{
        length_of_side: 4.,
    };

    println!("rectangle has area : {} and perimeter: {}", rectangle1.area(), rectangle1.perimeter());
    
    println!("Circle has area : {} and circumference: {}", circle1.area(), circle1.perimeter());

    println!("Square has area : {} and perimeter: {}", square1.area(), square1.perimeter());   
}



//ENUMS

enum Conveyance{
    Car(i32),
    Train(i32),
    Air(i32),
}

impl Conveyance{
    
    fn travel_allowance(&self) -> f32 {
        
        let allowance = match self{
            Conveyance::Air(miles) => *miles as f32 * 14. * 2.,
            Conveyance::Train(miles) => *miles as f32 * 18. * 2.,
            Conveyance::Car(miles) => *miles as f32 * 30. * 2.,
        };
        allowance
    }
}

fn enums() {
    let participant_1 = Conveyance::Car(60);
    let participant_2 = Conveyance::Train(200);
    let participant_3 = Conveyance::Air(1500);
    

    println!("Participant 1 has an allowance of £{} ", participant_1.travel_allowance());
    println!("Participant 2 has an allowance of £{} ", participant_2.travel_allowance());
    println!("Participant 3 has an allowance of £{} ", participant_3.travel_allowance());


}


#[derive(Debug)] //debug trait used for printing stuff
enum Value{
    Integer(i32),
    Float(f32),
}

fn vector_enum(){
    let some_val = vec![Value::Integer(12), Value::Float(2.0)];

    println!("The value of the integer vector is {:?} and the float value is {:?}", some_val[0], some_val[1]);

    for i in some_val.iter() {
        match i {
            Value::Integer(num) => println!("The value of the integer is {:?}", *num),
            Value::Float(num) => println!("The value of the float is {:?}", *num),
        }
    }   
}



//Generics

// fn square(x:i32) -> i32 { 
//     x.pow(2)
// }

// fn squaref32(x:f32) -> f32 {
//     x.powf(2.)
// }


//^^^^^^^^^^^^^^^^^^^^^^^
//You can use generics to avoid duplication

//writing restrictions which allow for generic function to run when only acceptable inputs are given for multiplication operation,
//(implements the traits of multiplication and copy), avoids compiler error.
//Primtives types are copied and not moved.

fn square<T: std::ops::Mul<Output = T> + Copy> (x:T) -> T {
    x*x
}

struct Point<T,U>{
    x:T,
    y:U,
}

impl<T: std::fmt::Debug,U: std::fmt::Debug> Point<T,U> {

    fn printing(&self){
        println!("the value of the point coordinates are {:?}, {:?}",self.x,self.y);
    }
}
fn point_struct(){
    let p1: Point<i32,i32> = Point{x:0, y:0};
    let p2: Point<f64,f64> = Point{x:1.0,y:4.0};
    let p3: Point<i32,f64> = Point{x:5, y:5.0};
    p1.printing()
}



//Option Enum

/*
enum Option<T
 */

fn option_enum(){
    // let mut disease:Option<String> = None;
    // disease = Some(String::from("Diabetes"));

    // match disease{
    //     Some(disease_name) => println!("You have the disease {}", disease_name),
    //     None => println!("You do not have a disease"), 
    // }

    let s1 = Some("Some String");
    println!("The value of s1 is {:?} and the value of s1 itself after unwrapping is {:?}", s1, s1.unwrap());


    let f1 = Some(10.54);
    let mut f2 = 16.5;
    f2 = f2 + f1.unwrap();

    println!("Sum = {}",f2);

    let number = Some(6);
    if square_numbers(number) != None {
        println!("The resukt of square is {:?}", square_numbers(Some(6)).unwrap());

    }

}
fn square_numbers(num: Option<i32>) -> Option<i32>{
    match num {
        Some(number) => Some(number*number),
        None => None,
    }
}

//Result enums

/*
enum Result<T,E> {
    Ok(T),
    Err(E)
}
 */

 fn division(divident: f64, divisor: f64) -> Result<f64,String> {
    // if divisor == 0.0 {
    //     Err(String::from("Error: division by zero"))
    // } 
    // else {
    //     Ok(divident/divisor)
    // }

    match divisor {
        0.0 => Err(String::from("Error: division by zero")),
        _ => Ok(divident/divisor),
    }

 }


 fn result_enums(){
    let some_vec = vec![5,5,2,1,5,9];
    let result =  match some_vec.get(5) {
        Some(a)=> Ok(a),
        None => Err("The value does not exist"),
    };
 }

 //Hash maps

 use std::{collections::HashMap, hash::Hash};

 fn hash_maps() {
    let mut person:HashMap<&str, i32> = HashMap::new();
    person.insert( "Prannvat",  40);
    person.insert("Kamran", 44);
    person.insert("shahid",  55);


    println!("The age is {:?}", person.get("Prannvat").unwrap());

    if person.contains_key("Prannvat") {
        println!("The value exists");
        
    }
    else{
        println!("The value does not exist");
    }


    match person.get("Kamran"){
        Some(value) => println!("The value exist {}", value),
        None => println!("The value does not exist"),

    }

    for (name, age) in &person{
        println!("the person {} is {}", name, age);
    }


    let mut likes:HashMap<&str, &str> = HashMap::new();
    likes.entry("Hubble").or_insert("apple");


    let some_vec = vec![1,2,3,4,5,6,7,8,9];
    let mut freq_vec: HashMap<i32, u32> = HashMap::new();
    
    for i in &some_vec{
       let freq =  freq_vec.entry(*i).or_insert(0);
        *freq += 1;
    }
    println!("{:?}", freq_vec)

 }



 #[derive(Debug)]
 struct Item{
    id: i32,
    title: String,
    year: i32,
    item_type: ItemType,

 }
 #[derive(Debug)]
 enum ItemType {
    Book,
    Magazine,
 }

 fn display_item_info(Item: Item)  {
    
    println!("{:?}", Item);
 }
 fn output_above(){
    display_item_info(
        Item{
            id: 0,
            title: String::from("ASOIAF"),
            year: 2010,
            item_type: ItemType::Book,
        }
    );
 }

 #[derive(Debug)]
 struct Student{
    id: i32,
    name: String,
    grade:String,
 }

 struct StudentManager{
    students:HashMap<i32, Student>,
 }

 impl StudentManager {
    fn new() -> Self {
        StudentManager{students: HashMap::new()}
    }

    fn add_student(&mut self, student: Student) -> Result<(), String> {
        if self.students.contains_key(&student.id) {
            Err(String::from(format!("Error: student {} already exists", student.id)))
        }
        else{
            self.students.insert(student.id, student);
            Ok(())
        

        }
    }

    fn get_student(&self, id: i32) -> Option<&Student>{
        self.students.get(&id)
    }  
 }

 fn run_student_assignment(){
    let student1 = Student{
        id: 0,
        name: String::from("Einstein"),
        grade: String::from("A"),
    };
    let student2 = Student{
        id: 0,
        name: String::from("Einstein"),
        grade: String::from("A"),
    };

    let mut student_mgr = StudentManager::new();
    match student_mgr.add_student(student1){
        Ok(()) => println!("{:?}", student_mgr.get_student(0).unwrap()),
        Err(error)  => println!("{}",error),
    }

    match student_mgr.add_student(student2){
        Ok(()) => println!("{:?}", student_mgr.get_student(0)),
        Err(error)  => println!("{}",error),
    }
   
   
    
 }
    

//Lifetimes
//Explains the scope for which a reference is valid

fn lifetimes(){
    let some_int = 10;
    let additional_int = some_fn(&some_int);
    println!("{}", additional_int);


   let int1 = 5;
   let int2 = 10;


   
}

fn some_fn(i:&i32) -> &i32 {
    &i
}
fn greater(i:&i32,j:&i32) -> i32 {
    if i > j{
        *i
    }
    else{
        *j
    }
}

//Closures

// |...| { ...}

fn closures(){
    let x = 5;
    let square = ||  println!("The square of the variable is {}", x*x);
    square();

    let print_info = |general_info:String, name : &str, age: i32|println!("{} {}", name, age);

    let general_info = String::from("The details are");
    let (person_name,person_age) = (String::from("Prannvat"), 18);
    print_info(general_info, &person_name, person_age);


    let square = |num| num*num;
    let x = 5;
    square(x);

    // let y = 105.5;
    // square(y); // won't work because first call sets types to i32.


    let division_status = |y: f32| {if y!= 0.0 {true} else {false}};

    division_closures_task(5.0, 10.2, division_status);


    let mut vec_1 = vec![1,2,3];

    let immut_some_closure = || { // immutable closure
        println!("Vec 1 : {:?}", vec_1);
    };

    let mut some_closure = || { //closure must be type mut as it is mutating a value inside of it
        vec_1.push(35);
        println!("Vec 1 : {:?}", vec_1);
    };
    
    // println!("Vec 1: {:?}", vec_1);
    // some_closure(); // called so scope finished so you can use immuatable reference below
    some_closure(); 
    vec_1.push(1);


    let vector_1 = vec![1,2,3];
    let mut move_value_some_closure = || { //closure must be type mut as it is mutating a value inside of it
       let vec_2 = vector_1;
    };
    move_value_some_closure();
    // println!("Vector 1: {:?}", vector_1);
    // println!("Vec 2 : {:?}", vec_2);
    //ownership transferred in closure so vec_2 no longer in scope and value is dropped so vector_1 not valid either

}

fn division_closures_task <F: Fn(f32) -> bool>(x:f32, y:f32, f: F)  {
    if f(y){
    println!("division result: {} ",x/y);
    }
    else{
        println!("division result is not possible");
    }
}

//Fuction types
fn fuction_types(){
    let mut f = min;
    println!("The minimum of two values is {}",f(2,3));

    let (my_name, my_age) = (String::from("Prannvat"), 18);
    prints_full_info(prints_name,&my_name, my_age);

    let answer = do_twice(add_one, 5);

}

fn max(x: i32, y: i32) -> i32{
    if x > y {x} else {y}
}

fn min(x: i32, y: i32) -> i32{
    if x < y {x} else{y}
}


fn prints_name(name:&str){
    println!("The name is {}",name);
}
fn prints_full_info(f: fn(&str), some_one: &str, age: i32) {
    f(some_one);
    println!("and my age is {}",age);

}

fn add_one(x: i32)-> i32 {
    x+1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}


//Iterators

fn iterators(){
    let some_vec = vec![1,2,3,4,5,6,7,8];
    let mut iter = some_vec.iter();

    println!("{:?}", iter.next());


    let a = vec![1,2,3,4,5,6,7];

    let mut check = a.iter().any(|&x| x> 0 ); //checks if atleast one element meets condition and returns true, else false
    let mut check2 = a.iter().all(|&x| x> 0 );// checks if all elements meet condition and return true, else false

    let check3 = a.iter().find(|&&x|x> 0);//finds first element that meets condition


    let check3 = a.iter().position(|&x|x> 4);//returns index position of first element that meet condition
    let check4 = a.iter().rposition(|&x|x> 4); // returns index position from the right of the and return first element that meet condition
    
    

    let a = vec![0,1,2,3,4,5,6,7,8,9];
    let filtered_values = a.iter().filter(|&x|*x>=5).collect::<Vec<&u32>>();
    println!("{:?}", filtered_values);
   
   let b = a.clone();
    let filtered_values = a.into_iter().filter(|&x|x>=5).collect::<Vec<u32>>(); //into_iter used actual values instead of references
    println!("{:?}", filtered_values);

    
    let mut mapped_value = b.iter().map(|x|2* *x).collect::<Vec<u32>>();
    println!("{:?}", mapped_value)

}

fn intersection(vec1: Vec<u32>, vec2: Vec<u32>) -> Vec<u32> {
    
    let intersection: Vec<u32> = vec1.iter().filter(|&x|vec2.contains(x)).cloned().collect();
    println!("Intersection of two vectors : {:?}", intersection);
    intersection

}

fn union(vec1: Vec<u32>,  mut vec2: Vec<u32>){
    
    let union = vec1.iter().filter(|&x|!vec2.contains(x)).collect::<Vec<&u32>>();
    
    for item in union{
        vec2.push(*item);
    }
    println!("Union of two vectors : {:?}", vec2);
}

fn assignment_ch_five() {
    let vec1: Vec<u32> = vec![1,2,3,4,5,6,7,8,9];
    let vec2: Vec<u32> = vec![1,2,3,4];

    intersection(vec1, vec2);
    let vec3: Vec<u32> = vec![1,2,3,4,5,6,7,8,9];
    let vec4: Vec<u32> = vec![1,2,3,10];
    union(vec3, vec4);    
}



