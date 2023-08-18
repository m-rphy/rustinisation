fn main() {

    let a = 5;

    if a > 5 {
        println!("a is bigger than 5");
    } else if a > 3 {
        println!("a is bigger than 3");
    } else {
        println!("a is smaller than or equal to 3");
    } 
    
    // return type of both branchs must be the same!
    let b = if a > 5 { 1 } else { -1 };

    // loops
    //
    // loop { this will loop forever }
    
    // you can name loops and break out of them with the "break" keyword

    'outer: loop {
        println!(" \"loop\" Loops forever unless broken with break statement");
        loop {
            break 'outer;
        }
    }

    // If you enclose a condition that could fail
    // (http request) you can enclose that functionality
    // in a loop and break out and assign the value 
    // with the break keyword
    let x = loop {
        break 5;
    };

    println!("x is: {}",x);

    // while loop
    let mut a = 0;

    // loop while the condition is true true
    while a < 5 {
        println!("a is: {}", a);
        a = a + 1;
    }

    

    // for loop
    
    let a = [1,2,3,4,5];
   
    // loop through an array
    for element in a {
        println!("{}", element);
    }

}
