use core::num;
use std::{io, cmp::Ordering};


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

// Before the loop, we declare a variable named counter and initialize it to 0. Then we declare a variable named result to hold the value returned from the loop. On every iteration of the loop, we add 1 to the counter variable, and then check whether the counter is equal to 10. When it is, we use the break keyword with the value counter * 2. After the loop, we use a semicolon to end the statement that assigns the value to result. Finally, we print the value in result, which in this case is 20.


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
}