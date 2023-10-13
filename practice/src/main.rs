// use core::num;
// use std::{io, cmp::Ordering};


/*

fn main() {
    // let  x=7;
    let mut x=8;
     println!("the value of x is {x}");
    //  x=9;    // ERR=> cannot mutate immutable variable x
    x=10;
     println!("value of x =>{x}")

}
 */


//  Shadowing 
 /*
 fn main(){
    let x=5;
    let x=x+1;
    {
        let x=x+2;
        println!("value of x in brackets {x}");  //8
    }
    println!("value of x outside brackets {x}");   //6

 }
 */


//  Tuples
/*fn main(){
    let _tup: (i32, f64, u8) = (500, 6.4, 1);    // best way differentiating types
    // destructuring 
    let tup1=(500,2.1,1);
    let (x,y,z)=tup1;
    println!("The destructured values {x}, {y}, {z}");

    // another way of assigning variables
    let first=tup1.0;
    let second=tup1.1;
    let third = tup1.2;
    println!("The values assigned are {first}, {second}, {third}");
}

 */

//  Array 
/* 
 fn main(){
 let a =[1,3,4,5,6,6];

     // access an element of an array that is past the end of the array.
    loop{
    let mut index =String::new();
    println!("enter a index you want to access");
    io::stdin()
    .read_line(&mut index)
    .expect("failed to read line");

    let index:usize =match index
    .trim()
    .parse(){
        Ok(num)=>num ,
        Err(_)=> continue,
    };
    let b:usize=6;
    match index.cmp(&b){
        Ordering::Less=> println!("good"),
        Ordering::Equal=> println!("good"),
        Ordering::Greater=> {
            println!("out of bound ");
            continue
        }
    }
    let element = a[index];
    println!("The element on {index} is {element}");

    }
 }


 */


// FUNCTIONS

/* 
fn main(){
    println!("main function");
    another_function(5,"children");
    let x= five();
    println!("value of x in main {x}");
}
fn five() -> i32 {
    5
}
fn another_function(x:i32,y:&str){
    println!("value of x is {x} and y is {y}");
}
*/

// CONTROL STATEMENTS

// fn  main() {
    // let number = 3;
//     loop{
//     let mut input = String::new();
//     io::stdin()
//     .read_line(&mut input)
//     .expect("msg");
//    let input:i32 = input
//     .trim()
//     .parse()
//     .expect("can't convert");
//     if input % 5==0{
//         println!("Divisible by 5 ");
//     }else if input %4 ==0{
//         println!("Divisible by 4");
//     }else if input %3 ==0{
//         println!("Divisible by 3");
//     }
//     else if input==89 {
//         break;
//     }
// }
//     let condition =false;
//     let y= if condition {56} else {84};
//     println!("{}",y);


// }

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
// //     };

//     println!("The result is {result}");
// }

// Before the loop, we declare a variable named counter and initialize it to 0. 
// Then we declare a variable named result to hold the value returned from the loop.
//  On every iteration of the loop, we add 1 to the counter variable, 
// and then check whether the counter is equal to 10. When it is, 
// we use the break keyword with the value counter * 2. After the loop, 
// we use a semicolon to end the statement that assigns the value to result. 
// Finally, we print the value in result, which in this case is 20.


// Returning Values from Loops

// fn main(){
//     let mut counter=0;
//     let result= loop{
//         counter+=1;
//         if counter==10{
//             break counter*2;
//         }
//     };
//     println!("the result is {result}");
// }




/*
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}*/

// fn main(){
//     let mut number =3;
//     while number !=0{
//         println!("{number}");
//         number-=1;
//     }
//     println!("LIFTOFF!!");
// }

// Looping Through a Collection with for

// fn main(){
//     let a=[10,20,30,40,50,60];
//     let mut index=0;
//     while index < 6{
//         println!("the number at index {index} is {} ",a[index]);
//         index+=1;
//     }
//     println!("Loop has ended!!");

// }


// 'For in' loop is more concise way of looping through collection

// fn main(){
//     let a=[10,20,30,40,50,60];
//     for element in a {
//         println!("the element is {}",element);
//     }
// }


// some range can also be used for 'For' loop
// fn main(){
//     for number in (0..89).rev(){
//         println!("{number}");
//     }
//     println!("LIFTOFF!!");
// }

// Reference and Borrowing 




// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     greet(m1, m2);
//     let s = format!("{} {}", m1, m2); // Error: m1 and m2 are moved
// }

// fn greet(g1: String, g2: String) {
//     println!("{} {}!", g1, g2);
// }

// In this example, calling greet moves the data from m1 and m2 into the parameters of greet.
//  Both strings are dropped at the end of greet, and therefore cannot be used within main.
//   If we try to read them like in the operation format!(..), then that would be undefined behavior. 
// The Rust compiler therefore rejects this program with the same error


// we can re-write tyhis program but it quiote inconventional




// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     let (m1_again, m2_again) = greet(m1, m2);
//     let s = format!("{} {}", m1_again, m2_again);
// }

// fn greet(g1: String, g2: String) -> (String, String) {
//     println!("{} {}!", g1, g2);
//     (g1, g2)
// }


// However, this style of program is quite verbose. 
// Rust provides a concise style of reading and writing without moves through references.

// References Are Non-Owning Pointers
// A reference is a kind of pointer. 
// Here's an example of a reference that rewrites our greet program in a more convenient manner:


// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     greet(&m1, &m2); // note the ampersands
//     let s = format!("{} {}", m1, m2);
// }

// fn greet(g1: &String, g2: &String) { // note the ampersands
//     println!("{} {}!", g1, g2);
// }


// The expression &m1 uses the ampersand operator to create a reference to (or "borrow") m1.
//  The type of the greet parameter g1 is changed to &String, meaning "a reference to a String".

// Observe at L2 that there are two steps from g1 to the string "Hello".
//  g1 is a reference that points to m1 on the stack, and m1 is a String containing a box that points to "Hello" on the heap.

// While m1 owns the heap data "Hello", g1 does not own either m1 or "Hello".
//  Therefore after greet ends and the program reaches L3, no heap data has been deallocated. 
// Only the stack frame for greet disappears. This fact is consistent with our Box Deallocation Principle.
//  Because g1 did not own "Hello", Rust did not deallocate "Hello" on behalf of g1.

// References are non-owning pointers, because they do not own the data they point to.




// Struct 

// struct User {
//     active:bool,
//     username:String,
//     email:String,
//     sign_in_count:i128
// }


// // Using the Field Init Shorthand

// fn build_user(email:String,username:String)->User{
//     User { active: (true), username, email, sign_in_count: (3) }
// }




// fn main(){

//     // Three ways of useing struct
//     let mut user1=build_user(String::from("shubhambhatt@123.com"), String::from("bhattji"));
//     let user2=User{
//         username:user1.username,
//         email:user1.email,
//         active:false,
//         sign_in_count:user1.sign_in_count
//     };
//     let mut user3 =User{
//         active:true,
//         ..user2
//     };
//     print!("{}",user3.email);
// }




// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }


// #[derive(Debug)]
// struct Rectangle{
//     width:i32,
//     height:i32
// }

// fn area( rectangle: &Rectangle)->i32{
//     rectangle.width*rectangle.height
// }


// fn main(){
//   let rect1=Rectangle{
//     width:32,
//     height:56
//   };
// //   let area=area(&rect1);
//   println!("{:#?}",rect1);
// }




// Method Syntax


// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }


// To define the function within the context of Rectangle, we start an impl (implementation) block for Rectangle.
//  Everything within this impl block will be associated with the Rectangle type. 
// Then we move the area function within the impl curly brackets and change the first (and in this case, only) 
// parameter to be self in the signature and everywhere within the body. 
// In main, where we called the area function and passed rect1 as an argument, 
// we can instead use method syntax to call the area method on our Rectangle instance. 
// The method syntax goes after an instance: we add a dot followed by the method name, parentheses, 
// and any arguments.



// impl Rectangle {
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     if rect1.width() {
//         println!("The rectangle has a nonzero width; it is {}", rect1.width);
//     }
// }



// Enum

enum IpAddress{
  V4(String),
  V6(String)
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage{
    x:i32,
    y:i32
}
struct WriteMessage(String);
struct ChangeColorMessage(i32,i32,i32);


impl Message {
    fn call(&self){
        
    }
}



fn main(){
    let mut four =IpAddress::V4(String::from("four"));
    let mut six=IpAddress::V6(String::from("six"));
   
}