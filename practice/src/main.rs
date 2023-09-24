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