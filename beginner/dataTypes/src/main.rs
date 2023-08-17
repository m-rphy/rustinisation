fn main() {

    // SCALAR DATA-TYPES

    // boolean
    let b1: bool = true;

    // unsigned integers
    let i1: u8 = 1;
    let i2: u16 = 1;
    let i3: u32 = 1;
    let i4: u64 = 1;
    let i5: u128 = 1;
    
    // signed integers
    let i6: i8 = 1;
    let i7: i16 = 1;
    let i8: i32 = 1;
    let i9: i64 = 1;
    let i10: i128 = 1;

    // floats
    let f1: f32 = 1.0;
    let f2: f64 = 1.0;

    // plateform specific ints
    let p1: usize = 1;
    let p2: isize = 1;

    // characters. &str, and string
    
    // single character
    let c1: char = 'c';
    // string slice -> all string literals are string slice
    let s1: &str = "hello";
    // Learn about this later
    let s2: String = String::from("world");

    // COMPOUND DATA-TYPES

    // Arrays
    //
    // hold multiple values of the same type
    //  rust will infer the type as well
    let a1: [i32; 5] = [1,2,3,4,5];

    // grabbing a value from the array
    let i1: i32 = a1[4];

    // Tuples
    //
    // Tuples hold multiple values of the
    //   same or different data types
    let t1: (i32, i32, i32) = (1,2,3);
    let t2: (i32, f64, &str) = (1, 2.0, "3");

    // grabbing a value from a tuple
    let s1 = t1.2;

    // tuple destructuring
    let (i1, f1, s1) = t2;
    
    // An empty tuple is called a unit type
    let unit = ();

    // Functions that don't return a value
    // implicitly return a unit type
    

    // Type Aliasing
    type age = u8;

    let a1: age = 57;

}
