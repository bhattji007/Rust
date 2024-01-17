


// -------------------------------------<Section Starts>--------------------------------------------------

// mod main_hall{                                 
//     pub mod hosting {                                  
//        pub fn add_to_waitlist() {}                                
//         fn seat_at_table() {}                                
//     }                                  
//     pub mod serving {                                   
//         fn take_order() {}                                
//         pub fn serve_order() {}                                
//         fn take_payment() {}                                   
//     }                                  
// }                                  


// pub fn eat_at_restraunt(){
//     // Absolute path
//     crate::main_hall::hosting::add_to_waitlist();  // 


//     // Relative path
//     main_hall::hosting::add_to_waitlist();
// }


// mod kitchen{
//     fn fix_incorrect_order(){
//         cook_order();
//         super::main_hall::serving::serve_order();
//     }
//     fn cook_order(){}
// }

// --------------------------------------<Section Ends>--------------------------------------------------




// -------------------------------------<Section Starts>--------------------------------------------------


// mod kitchen {
//     pub struct Breakfast{
//         pub toast: String,    // by default struct fields are private
//         seasonal_fruit: String,
//     }
//     impl Breakfast{
//         pub fn summer(toast:&str)->Breakfast{           // summer function is associated with Breakfast struct
//             Breakfast{                          // struct Breakfast is returned
//                 toast:String::from(toast),          // toast is a String
//                 seasonal_fruit:String::from("peaches"),     // seasonal_fruit is a String
//             }
//         }
//     }
// }

// pub fn eat_at_restraunt(){
//     let mut meal=kitchen::Breakfast::summer("french toast");

//     meal.toast=String::from("wheat bread");
// }




// mod kitchen{
//     pub enum Appetizer{
//         Soup,                               // enum variants are public by default
//         Salad,
//     }
// }

// pub fn eat_at_restraunt() {
//     let order1= kitchen::Appetizer::Soup;
//     let order2= kitchen::Appetizer::Salad;
// }

// --------------------------------------<Section Ends>--------------------------------------------------

// -------------------------------------<Section Starts>--------------------------------------------------


// mod front_of_house{
//     pub mod hosting{
//         pub fn add_to_waitlist(){}
//     }
// }


// use self::front_of_house::hosting;  // use keyword brings a path into scope

// pub fn eat_at_restraunt(){
    
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();   // we can call the function without the front_of_house prefix
//     hosting::add_to_waitlist();

// }

// --------------------------------------<Section Ends>--------------------------------------------------


// use std::fmt;
// use std::io;


// fn function1() -> fmt::Result{   // fmt::Result is a type alias for Result<T, fmt::Error>
//     Ok(())
// }

// fn function2() -> io::Result<()>{  // io::Result is a type alias for Result<T, io::Error>
//     Ok(())
// }

// Both of these functions return a value of type Result<T, E> where T is () and E is the error type specified in the type alias.


// Another way of specifying the path could be 

// use std::fmt::Result;   // Result is now Result<T, E>
// use std::io::Result as IoResult;   // Result is now IoResult<T>

// fn function1() -> Result{
//     Ok(())
// }
// fn function2() -> IoResult<()>{
//     Ok(())
// } 


// --------------------------------------<Section Ends>--------------------------------------------------


// -------------------------------------<Section Starts>--------------------------------------------------



// mod main_hall{
//     pub mod hosting{
//         pub fn add_to_waitlist(){}
//     }
// }



// // use rand::Rng;  // use keyword brings a path into scope
// // use rand::ErrorKind::Transient;  //ErrorKind is an enum
// // use rand::CryptoRng;  // CryptoRng is a trait


// // or we can use nested paths

// use rand::{Rng,CryptoRng,ErrorKind::Transient}; // nested paths

// pub use crate::main_hall::hosting;  // pub use makes the path available to code that calls our code outside of the crate

// pub fn eat_at_restraunt(){

//     let secret_number = rand::thread_rng().gen_range(1, 101);  // rand::thread_rng() returns a type that has the gen_range method on it
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }   


// --------------------------------------<Section Ends>--------------------------------------------------

// -------------------------------------<Section Starts>--------------------------------------------------
// importing modules from other files
mod main_hall;

pub use crate::main_hall::hosting;

pub fn eat_at_restraunt() {
        hosting::add_to_waitlist();
}