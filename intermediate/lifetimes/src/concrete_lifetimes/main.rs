
// On of the memory safety garantees is that there are no
// dangling references

// references must be valid at compile time


// A concrete lifetime is the time during which
// a value exisist at a particular location

// It starts when a value is added a particular
// memory location and ends when that value is 
// dropped /moved out that memory location



fn main() {
    
    let r2;

    // the lifetime of s1 starts here
    let s1 = "Hello from concrete".to_owned();
    {
        // if s1 was defined here, it would be dropped 
        // at the end of this scope

        let s2 = "first liftime".to_owned();
        r2 = &s2;

        println!("r2: {}", r2); 
        // You can't use a reference to s2 outside of this scope
        // because it's lifetime is dropped
    }

    // Lifetime of s1 ends here
    println!("s1: {}", s1);

    let mut s3 = "non-lexical-liftimes".to_owned();

    let r3 = &s3; 
    println!("r3: {r3}");

    let r4 = &mut s3;
    r4.push_str("!");
    println!("r4: {}", r4);

    // Although it appears that r3 and r4 are breaking
    // the rules, the lifetimes of the references don't
    // over lap.
    //
    // r3 lived from line 40 to 41
    // while r4 lives from 443 to 45
    //
    // If r3's lifetime was enveloped by r4's lifetime
    // the barrow checker would not allow the code to compile
}



