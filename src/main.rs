#![allow(unused)]

use rnp::rar;


rar!(Array, 5);
fn main(){
    println!("Hello rnp!");

    let a = Array::new([1.0, 2.0, 3.0, 4.0, 5.0]);
    let b = Array::new([6.0, 7.0, 8.0, 9.0, 10.0]);

    let sum = a.add(&b);
    let diff = a.sub(&b);
    let dotp = a.dot(&b);

    println!("dot product of the arrays: {}", dotp);
    
    // formatting error in below code
    // println!("{:?}", diff);

}