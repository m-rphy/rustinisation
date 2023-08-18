
fn main() {
    let z = my_func(22);
    println!("my_func returned: {}", z);
}

fn my_func(x: i32)-> i32 {
    // print line statement
    // to return early
    //  -> return 5;
    println!("my_function called with: {}", x);
    let y = 10;
    // the final expression is the return value
    // you have to omit the semi-colon too!
    y
}

