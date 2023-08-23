use std::sync::Arc;

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

    palindrome();
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
        palindrome_vec.push(c);
    }

    let mut palindrome_check : Vec<char> = vec![];
    for element in &mut palindrome_inp.chars().rev(){
        palindrome_check.push(element);
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

