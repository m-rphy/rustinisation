
// If you are passing in owned types, you
// don't need to worry about lifetimes, becuase the
// lifetime of the content is the same as the 
// lifetime of the Struct

// However, when storing references in the a struct
// you need to specify the lifetime of the reference
struct Tweet<'a> {
    content: &'a str,
}

// Just like with generics we need to specify that
// we are using generic lifetimes

// Furthermore, we need to specify the relationship
// of the lifetime in the new_content
impl<'a> Tweet<'a> {
    fn replace_content(&mut self, new_content: &'a str) -> &str {
        let old_content = self.content;
        self.content = new_content;
        old_content
    }
}

fn main() {
    let mut tweet = Tweet{
        content: "This is a tweet"
    };
    tweet.replace_content("This is another tweet");

    println!("What did I tweet? {}", tweet.content);
}
