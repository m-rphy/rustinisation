fn main() {
    
    // Must use explict Type annotation when
    // using the new key word
    let _v: Vec<String> = Vec::new();

    // implicit type annotation is accomplished
    // with mutable vectors
    let mut v_mut = Vec::new();

    // When you push a value into a vector
    // The vector has ownership of it's element
    v_mut.push(String::from("Hello"));
    v_mut.push(String::from("There"));
    v_mut.push(String::from("World"));


    // Using the vec macro (nicer imo)
    let _v_2 = vec![1,2,3];

    // Indexing into a vector
    
    let _s = &v_mut[0]; // can panic is there is an invalid index
                       //
                       // You can't move an element with this syntax
    
    let _s = v_mut.get(0); // Won't panic -> Returns an Option Enum

    if let Some(el) = _s {
        println!("{el}"); // -> Hellow
    }

    // To move an element out of a vector
    // let s = v_mut.remove(0); // RE-indexes the vector

    // Iterating over a vector 
    // NOTE - Mutably barrowing v_mut
    for s in &mut v_mut {
        s.push('!');
    }

    // Just barrowing
    for s in &v_mut {
        println!("{s}");
    }

    // Taking ownership of values from v_2
    let mut v_3 = vec![];
    
    for el in _v_2 {
        v_3.push(el);
    }

    // The ownership of the v_2 elements 
    // is moved to the v_3 vector

    
}

