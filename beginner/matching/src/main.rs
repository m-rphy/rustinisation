
// Match is a powerful control flow operator 
// Allowing for you to compare a value against a series of
// patterns to determine which code block to execute

// Similar to switch

enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace{
        from: String,
        to: String
    }
}

impl Command {
    fn serialize(&self) -> String {
        match self {
            Command::Undo => String::from(
                "{\"cmd\": \"undo\" }"
                ),
            Command::Redo => String::from(
                "{\"cmd\": \"redo\" }"
                ),
            Command::AddText(s) => {
                format!(
                    "{{\
                    \"cmd\": \"add_text\" , \
                    \"text\": \"{s}\" \
                    }}"
                    )
            },
            Command::MoveCursor(x, y) => {
                format!(
                    "{{\
                    \"cmd\": \"move_cursor\" , \
                    \"line\": \"{x}\" , \
                    \"column\": \"{y}\" \
                    }}"
                    )
            },
            Command::Replace { from, to } => {
                format!(
                    "{{\
                    \"cmd\": \"replace\" , \
                    \"from\": \"{from}\" , \
                    \"to\": \"{to}\" \
                    }}"
                    )
            }
        }
    }
}




fn main() {
    let age = 33;

    match age {
        1 => println!("Happy 1st Birthday"),
        13..=19 => println!("You are a Teenager"),
        // -----catch-all patterns-----
        // Binding the value
        x => println!("You are {x} years old"),
        // Does not bind the value
        _ => println!("")
    }

    let undo = Command::Undo;
    let redo = Command::Redo;
    let move_cursor = Command::MoveCursor(11, 81);
    let replace = Command::Replace{
        from : String::from("Billy"),
        to : String::from("Billy & Hannah")
    };
    let add_text = Command::AddText(String::from("text added"));

    println!("{}",undo.serialize());
    println!("{}",redo.serialize());
    println!("{}",move_cursor.serialize());
    println!("{}",replace.serialize());
    println!("{}",add_text.serialize());

}
