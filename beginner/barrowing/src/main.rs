
// Barrowing is the act of creating a reference
// to a value (similar but not equal to pointers)

//  --- Reasons to Barrow ---
// 1. A lot more performance! (cheaper than cloning)
// 2. Ownership might not be needed!

//         --- Rules ---
// 1. You can only have 1 mutable reference
//      (any number of immutable references)
// 2. Refrences must always be valid

// rule 1 prevents data races
// rule 2 prevents dangling references


fn main() {
    let s0 = String::from("Rust");
    
    // Barrowing!
    // immutable reference
    let r0 = &s0;
    
    println!("non lexical lifetimes allow for s0 to be");
    println!("printed here: {s0}...\n and refrenced below");
    
    print_string(r0);

    // can solve the error with shadowing
    // NOTE: not efficient or necessary
    
    // add_string take ownership of s0, mutates and 
    // returns a the newly created mutated string
    let s0 = add_string(s0); // -> Rust is awesome!

    // -----------------------------------------------

    // Instead create a mutable string variable
    let mut s1 = String::from("Again, Rust");
    
    // create a mutable refrence to that variable
    let r1 = &mut s1;

    // and mutable the refrence
    //  (You can also return a mutable reference
    //  as well!
    let mut s2 = add_to_string_ref(r1);

    // (The variable being referenced must be mutable)
    // mutable references
    let r1 = &mut s2;

    println!("Cannot print s2 here though. It owned by r1");
    
    println!("s0 is the orginal string, mutated in a function and shadowed");
    println!("to become: {s0}");

    println!("r1 is a mutable reference to a mutable string and is: {r1}");
}

fn print_string(p1: &String) {
    println!("{p1}");
}

fn add_string(mut p1: String) -> String {
    p1.push_str(" is Awesome!");
    p1
}

fn add_to_string_ref(p1: &mut String)-> &mut String {
    p1.push_str(" is awesome!");
    p1
}
