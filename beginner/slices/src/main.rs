fn main() {
    
    // Slices are references to a contiguous
    // sequence of elements in a collection

    // You want to reference a part of a string
    let tweet = String::from(
        "This is my tweet and it is ver very long"
        );
    // from the beginning to 20 characters
    let trimmed_tweet: &str = &tweet[..20];

    let trimmed_tweet_2 = trim_tweet(&tweet);

    let tweet_2 = "This is my tweet and it is ver very long";
    let trimmed_tweet_3: &str = trim_tweet(&tweet_2);


    println!("{trimmed_tweet_2}");
    println!("{trimmed_tweet_3}");
    // &str is a string slice! 
    //  bc has an unknown length is
    //  is a reference to str (immutable data type)
    //  hence &str are also immutable
    println!("{trimmed_tweet}");


    // Slices can also work with arrays!
    
    let arr = [1,2,3,4,5];
    let arr_slice = &arr[..3];

    // "{:?}" -> debug formatting
    println!("{:?}", arr_slice);
}

// You can past &String and &str to this function
//  --> De-ref coersion (&String -> &str)
fn trim_tweet(s: &str) -> &str {
    &s[..20]
}
