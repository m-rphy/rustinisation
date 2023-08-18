
// __________OWNERSHIP RULES_____________
//  
// 1. Each value in Rust has a variable
//      and that variable is called the owner.
//
// 2. There can only be one owner at a time
//
// 3. When the owner goes out of scope, the 
//      value will be dropped.
//

// ________SOLVES________
//
// * Memory/Resource Leaks
// * Double Free Errors
// * Use After Free Errors

fn main() {

    let s1: String = String::from("Rust"); // heap allocated string
    {
        let s0: String = String::from("Inner Scope"); // heap allocated string
    } // s0 is dropped

    // The ownership of s1 will pass the function's scope
    // once the function is finished executing the 
    // the memory on the heap with be de-allocated
    
    //print_string(s1);
    // s1 isn't pointing to anything on the heap is used after this point

    // Passing Ownership into a function
    // to make the function work... pass a clone of s1
    print_string(s1.clone());

    println!("s1 is: {s1}");

    // s2 is now pointing to the heap allocated memory
    let s2: String = s1;
    // We can only have 1 owner (pointer) to the heap allocated string
    // println!("s1 is : {s1}"); would cause error

    // s3 is pointing to it's own heap allocated string on the heap
    let s3 = s2.clone();
  
    println!("s3 is a copy of s2, equaling: {s3}");
    println!("s2 is: {s2}");

    // Returning ownership
    //
    // owner ship of the returned value is passed to s4
    let s4 = generate_string();
    println!("s4 is: {}", s4);


    // Passing and Returning ownership
    let s5 = add_to_string(s2);
    println!("{s5}"); // -> "Rust is awesome!"
    
    // shadowing s5
    let s5 = add_to_string(s4);
    println!("{s5}"); // -> "Baz is awesome!"




    // Ownership is different from primitative data types
    
    // These are cheap to clone and done for you under the hood
    // (ints, chars, floats, bools)
    let x = 10;
    let y = x;

    // This works though!!
    print_int(x);
    println!("x is: {x}. y cloned x and is: {y}")
    
    // passing variables to function

}

fn print_int(i: i32) {
    println!("i is: {i}");
}


fn print_string(p1: String) {
    println!("{p1}");
    // any string passed into this function
    // will be dropped once this function
    // is finished executing and the allocated
    // memory on the heap will be dropped
}

fn generate_string() -> String {
    String::from("Baz")
}

// must pass a mutable variable
fn add_to_string(mut p1: String) -> String {

    p1.push_str(" is awesome!");
    p1
}
