
//Here we use the two crates file1 and file2
//Crates can be either binary crates or library crates
//We can only have 0 or 1 library crates ie lib.rs
//Library crate is used to organise functionality and is not meant to be executed.
//Binary crates are made to be executed

pub mod file1;
pub mod file2;


//Library, which is used to share functionality and organises crates and modules.
//By deafult every crate has a single module and that module is the crate itself.


