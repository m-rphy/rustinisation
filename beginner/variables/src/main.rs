fn main() {
    
    // Creation of a variable
    
    // rust infers the type automatically
    let a = 5;
    println!("a is: {a}"); // -> a is: 5
    
    // or declare eplicitly
    let b: i32 = 5;
    println!("b is: {b}"); // -> b is: 5
    
    // Mutability
    
    // You must state that the variable is mutable 
    // explicitly with the "mut" keyword
    
    let mut c = 1;
    println!("c is: {c}"); // -> c is: 1
    
    c = 10;
    println!("c is: {c}"); // -> c is: 10
    
    c = 100;
    println!("c is: {c}"); // -> c is: 100

    // Shadowing
    
    // Bc these variables have the
    // same name and are within the
    // same scope the following "c"
    // will shadow the preceding "c"
    // and be re-assigned
    
    let d = 10;

    println!("d is : {d}"); // d is: 10
    
    let d = 20;
    
    // print a formatted string
    println!("d is : {d}"); // -> d is: 20

    // Scope
    let e = 40;
    {
        // if you define a variable within
        // this scope and call is in the outer
        // scope you will get a compile time error
        let e = 30;
        println!("inner scope e is: {e}") // inner scope e is: 30
    }

    // within the same scope as e == 30
    println!("outer scope e is: {e}"); // e is: 40

    
}
