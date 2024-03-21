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

// Any IP address can be either a version four or a version six address,but not both at the same time.
//  That property of IP addresses makes the enum data structure appropriate because an enum value can only be one of its variants.
//  Both version four and version six addresses are still fundamentally IP addresses, 
// so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.

// We can express this concept in code by defining an IpAddrKind enumeration and listing the possible kinds an IP address can be,
//  V4 and V6. These are the variants of the enum:





// enum IpAddress{
//   V4(String),
//   V6(String)
// }
// // creating a structure consisting the kind of IpAddress and the address value
// struct IpAddr {
//     kind: IpAddress,
//     address: String,
// }
// enum Message{               // enum can also have methods
//     Quit,
//     Move{x:i32,y:i32},
//     Write(String),
//     ChangeColor(i32,i32,i32)
// }
// impl Message{
//     fn call(&self){
//         // method body

//         println!("The message is hello world",);
//     }
// }

// fn main(){
//     // let mut four  = IpAddress::V4(String::from("localhost"));
//     // let mut six = IpAddress::V6(String::from("localhost"));
//     Message::call(&Message::Quit);
//     Message::call(&Message::Move{x:5,y:6});
//     Message::call(&Message::Write(String::from("Hello world")));
//     Message::call(&Message::ChangeColor(5,6,7));

//     // let mut four =IpAddress::V4(String::from("four"));
//     // let mut six=IpAddress::V6(String::from("six"));
//     // println!("{:#?}",four);
// }



// Option Enum and its advantages

// fn main(){
//     // let number :Option<i32>=Some(5);
//     // let string :Option<&str>=Some("Hello world");
//     // let absent_number:Option<i32>=None;

//     let x:i8=5;
//     let y:Option<i8>=None;   // None is of type Option<i8> 
//     let sum=x+y.unwrap_or(0);  // unwrap_or() is a method of Option<T> which returns the value of T if it is Some(T) else returns the value passed as argument
//     // let sum=x+y;  // this will give error as y is of type Option<i8> and x is of type i8
// }


// Match Control Flow Operator


// #[derive(Debug)]   // this is used to print the enum values

// enum UStates{
//     Alabama,
//     Alaska,
//     Arizona,
//     Arkansas,
    
// }


// enum Coin{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UStates) // this is an enum inside an enum
// }

// fn value_in_cents(coin:Coin)->u32{
//     match coin{
//         Coin::Penny=>{
//             println!("Lucky Penny");
//             1
//         },
//         Coin::Nickel=>5,
//         Coin::Dime=>10,
//         Coin::Quarter(state)=>{
//             println!("State quarter from {:?}",state);
//             25
//         }
//     }
// }

// fn main(){
//         value_in_cents(Coin::Quarter(UStates::Alaska));

// }




// combining match expression with enum 
// fn main(){
//     let five =Some(5);   // Some is an enum
//     let six=plus_one(five);  
//     let seven=plus_one(six);   // plus_one() is a function which takes an Option<i32> and returns an Option<i32>
//     let eight=plus_one(Some(78));
//     println!("value of eight {:#?}",eight.unwrap()); 
//     println!("value of  seven {:#?}",seven.unwrap()); 
//     println!("value of six {:#?}",six.unwrap());
//     println!("value of five {:#?}",five.unwrap());   
// }

// fn plus_one(x:Option<i32>) -> Option<i32>{  // this function takes an Option<i32> and returns an Option<i32>
//     match x{
//         None=>None,    // if x is None then return None
//         Some(5)=>Some(6+1),   // if x is Some(i) then return Some(i+1)
//         _=>Some(7867)
//     }   
// }

// fn main(){
//     // let a =[1,2,3];
//     // let mut  v:Vec<i32>= Vec::new();    // one way to initialize vectors

//     // v.push(1);
//     // v.push(2);
//     // v.push(3);
//     // println!("hello");


//     let mut v2= vec![1,2,63,4,5];  //another way to initialize vector 
//     // let third =&v2[2];
//     // println!("the third element is {}",third);


//     // another way to access the vector values 

//     // match v2.get(2){
//     //     Some(third)=> println!("the third value is {}",third),
//     //     None=>println!("there is no such index")
//     // }


//     // Iterating over vectors 
//     for i in &mut v2{
//         println!("{}",*i+1);            // we use (*) to dereference the integer and use underlying value 
//     }
// }   



// fn main(){
//     // Strings
//     // different ways to define strings
//     // let s1=String::new();
//     // let s2="initial Content";
//     // let s3=s2.to_string();
//     // let s4 =String::from("This is the string i was talking about ");


//     // Appending to Strings

//     // let mut s=String::from("Hello");
//     // s.push_str( " world ");
//     // s.push('!');

//     // println!("the String is {}",s);

//     // let s1=String::from("Hello ");
//     // let s2=String::from("WORLD ");
//     // let s4=format!("{}{}",s1,s2);
//     // let s3=s1+ &s2;
//     // println!("the String is {} and {}",s4,s3);



//     // // Indexing into a string
//     // let hello=String::from("hellow world !!");
//     // for c in hello.chars(){
//     //     println!("{}",c);
//     // }

// }



// HASHMAP


// use std::collections::HashMap;

// fn main(){
//   let blue =String::from("Blue");
//   let yellow=String ::from("Yellow");
//   let mut scores= HashMap::new();
//     scores.insert(blue,10);
//     scores.insert(yellow,13);
// }


// Error Handling


// fn main(){
//     panic!("crash and burn");       // if your program cannot handle error gracefully then you can use panic 
// }


// BackTrace in RUST


// fn main(){
//     a();
// }

// fn a(){
//     b();
// }

// fn b(){
//     c(22);
// }
// fn c(num:i32){
//     if num==22{
//         panic!("Not the 22 please!");
//     }
// }



// Recoverable Errors in RUST

// fn main(){
//     enum Result <T,E>{
//         Ok(T),
//         Err(E)
//     }
// }

// use std::fs::File;

// use std::io::ErrorKind;

// fn main(){
//     let f=File::open("/demo.txt");
//     // let _f=match f{
//     //     Ok(file)=>file,
//     //     Err(error)=> match error.kind(){
//     //         ErrorKind::NotFound=> match File::create("demo.txt"){
//     //             Ok(fi)=>fi,
//     //             Err(e)=>panic!("There was a problem creating the file {:?}",e)
//     //         },
//     //         other=>{
//     //             panic!("There was a problem openming the file {:?}",other)
//     //         }
//     //     }
//     // };



//     //Easy way of write the same code of opening a file gracefully 
//     let f1=File::open("demo.txt").unwrap();
//     // or
//     let f3=File::open("deo.txt").expect("file can't be open");

// }



// Error Propogation

// use std::fs::File;
// use std::io;
// use std::io::Read;

// fn read_text_from_file()->Result<String,io::Error>{
//     let f=File::open("demo.txt");
//     let mut f= match f{
//         Ok(file)=>file,
//         Err(er)=>panic!("problem while opening the file {:?}",er),
//     };
//     let mut s= String::new();
//     match f.read_to_string(&mut s){
//         Ok(_)=>Ok(s),
//         Err(e)=>Err(e)  
//     }
// }



// Generic Types


// fn main(){

//     let number_list =vec![10,20,30,40,50,60];
//     let mut largest=number_list[0];
//     for num in number_list{
//         if num>largest{
//             largest=num;
//         }
//     }
//     let number_list =vec![10,20,30,40,90,100,1091,50,60];

//     let function= get_largest_num(number_list);


//     let char_list=vec!['y','u','e','s'];
//     let largest_char= get_largest_char(char_list);
//     println!("The largest number is {} and {} and {}",largest, function,largest_char);

// }


// // lets create a fucntion for it 

// fn get_largest_num (number_list:Vec<i32>)->i32{
//     let mut number= number_list[0];
//     for num in number_list{
//         if num >number{
//             number= num;
//         }
//     }
//     number
// }


// fn get_largest_char (number_list:Vec<char>)->char{
//     let mut number= number_list[0];
//     for num in number_list{
//         if num >number{
//             number= num;
//         }
//     }
//     number
// }


// fn get_largest_all<T:PartialOrd+Copy>(number_list:Vec<T>)->T{  //generic used
//     let mut number= number_list[0];
//     for num in number_list{
//         if num >number{
//             number= num;
//         }
//     }
//     number
// }

// struct Point<T,U> {
//     x:T,
//     y:U
// }


// fn main(){
//     let p1 =Point{x:5,y:67};
//     let p2 =Point{x:5.0,y:67.0};
//     let p3 =Point{x:5.0,y:67};   // Multiple generic types in a single struct in this situation
// }



// Generics inside enums 

// fn main(){
// enum Option<T>{
//     Some(T),
//     None,
// }
// enum Result<T,E>{
//     Ok(T),
//     Err(E)
// }
// }


// Implementation of structs



// struct Point<T> {
//         x:T,
//         y:T
//     }
    
//     impl <T> Point<T>{
//         fn x(&self)->&T{
//             &self.x
//         }
//     }

//     impl  Point<f64>{
//         fn y(&self)->f64{
//             self.y
//         }
//     }
// fn main(){
//      let p=Point{x:5,y:10};
//      p.x();
//      let p1=Point{x:5.0,y:10.0};
//      p1.y();
// }

// struct Point<T,U>{
//     x:T,
//     y:U
// }

// impl <T,U> Point<T,U> {
//     fn mixup<V,W>(self,other:Point<V,W>)-> Point <T,W>{
//         Point {
//             x:self.x,
//             y:other.y
//         }
//     }
// }

// fn main(){


//     let p=Point{x:5,y:10};
//     let p1=Point{x:"Hello",y:'c'};
//     let p3=p.mixup(p1); 
//     println!("p3.x= {}, p3.y={}",p3.x,p3.y);
// }