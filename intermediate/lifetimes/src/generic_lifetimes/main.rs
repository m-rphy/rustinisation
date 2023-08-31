
fn main() {
    println!("Hello from generic");

    let player_1 = "player_1".to_string();
    let player_2 = "player_2".to_string();

    {
        
        // 'a is the lifetime of player_3 becuase it's the shortest
        // lifetime and hence another_result has the same lifetime of
        // player_3
        let player_3 = "player_3".to_string();
        let another_result = fn_call_1(player_1.as_str(), player_3.as_str());
        println!("another_result: {}", another_result);
    }

    // 'a is the lifetime of player_2 becuase it's the shortest lifetime
    // and thus the lifetime of result is the same as player_2
    let result = fn_call_1(player_1.as_str(), player_2.as_str());
    
    println!("result: {}", result);

}

// What is p1 and p2 had different lifetimes?
// The compiler wouldn't know is the reference was 
// still in scope and we could end up with a dangeling reference.
//
// What is the lifetime of the return reference?

// This is why there are generic lifetime annotations "<'a>"

// Generic lifetimes, they specify what the lifetime of the return
// refrence is!
// They are not concrete lifetimes, but describe the relationship 
// between concrete lifetimes

fn fn_call_1<'a>(p1: &'a str, p2: &'a str) -> &'a str {    

    if rand::random()  {
        p1
    } else {
        p2
    }
}
