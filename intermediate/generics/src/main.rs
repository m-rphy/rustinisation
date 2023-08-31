
// Generics allow you define structs, enums
//  and funcitons with generic types that will be
//  substiuted with concrete types at compile time


//  --monomorphization--
// There is not runtime cost when using generics
//  because rust will create as many copies of the function
//  as there are types are passed into that function at
//  compile time -> This is call monomorphization
//
//  (Also occurs with structs, enums, and impl_blocks)

// TL;DR -> If you wan to have a function that
//          deals with mutiple types, use a generic
//          typ

// NOTE
// we could also use cammelCase (by convention) for a generic
//  --> <T> === <payLoadType>

struct BrowserCommand<T> {
    name: String,
    payload: T,
}

// Generics in impl_blocks

// We need to declare the generic twice so the 
// compiler knows that we are implementing 
// functionality for the BrowserCommand struct
// of any type

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand {
            name,
            payload,
        }
    }

    // returning a generic
    //
    // Doesn't take ownership!
    fn get_payload(&self) -> &T {
        &self.payload
    }
}

// impl block with concrete types

// we can print it bc we defined it to be a string!
// So we couln't call this command on a type that
// isn't a string
impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}

// You can also use generics in enum definitions
// (Just like Option and Result enums)

// ... and in free functions!

fn serialize_payload<T>(payload: T) -> String {
    // conver to JSON string...
    "placeholder".to_owned()
}

// The definition of the Result Enum
enum Result<T,E> {
    Ok(T),
    Err(E)
}

fn main() {
    let cmd_1 = BrowserCommand {
        name: "navigate".to_owned(),
        payload: "localhost:7878".to_owned(),
    };

    let cmd_2 = BrowserCommand {
        name: "zoom".to_owned(),
        payload: 200
    };

    let cmd_3 = BrowserCommand::new(
        String::from("click"),
        String::from("event.target.id")
        );

    let p1 = cmd_1.get_payload();
    let p2 = cmd_2.get_payload();
    let p3 = cmd_3.get_payload();

    // passing into different type
    let j_1 = serialize_payload(p1);
    let j_2 = serialize_payload(p2);

    // can do this becuase cmd_1 has a payload of type String
    cmd_1.print_payload();
}
