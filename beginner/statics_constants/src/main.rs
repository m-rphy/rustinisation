
// need to have explicit type annotation
// can be delcared anywhere in the file
const MAX_PLAYERS: u8 = 10;

static CASINO_NAME: &str = "RUSTY'S";

// you can make static variable mutable
// but have to do it in unsafe mode

fn main() {

    // constants don't have
    // a specific place in 
    // memory. Meaning there
    // value will be in-lined
    let a = MAX_PLAYERS; // => inlined to be let a: i32 = 10
    let b = MAX_PLAYERS;

    // statics do have a specific 
    // place in memory 
    // Default to constants
    let c = CASINO_NAME;
    let d = CASINO_NAME;

    // use statics judiciously
    // like when you need the single
    // address property
    // or are dealing with large
    // amounts of data and would 
    // benefit from not copying
    // the data directly but rather
    // only passing the address
}
