
struct Product {
    name: String,
    //category: String, // not the best solution
    category: ProductCategory, 
    price: f32,
    in_stock: bool
}

enum ProductCategory {
    Books,
    Clothing,
    Electronics

} 

///////////////////////////////////////////
// enum commands

// Commands for a text editing software
// variants can contain data too!
enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String
    }
}

// we can also creat impl blocks for an enum
// (Why is so cool if we have structs)

impl Command {
    fn serialize(&self) -> String {
        String::from("JSON")
    }
}



fn main() {

    // grabbing the value from the enum
    let category = ProductCategory::Electronics;

    let product = Product {
        name: String::from("iPod"),
        category,
        price: 230.00,
        in_stock: true

    };

    //////////////////////////////////
    let cmd = Command::Undo;
    let cmd = Command::Redo;
    let cmd = Command::AddText(String::from("Rust"));
    let cmd = Command::MoveCursor(11, 81);
    let cmd = Command::Replace{
        from: String::from("a"),
        to: String::from("b")
    };
    
    let json_string = cmd.serialize();

}
