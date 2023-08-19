

// What the Option Enum looks like

/*
   enum Option<T> {
       None,
       Some(T)
   }
*/

fn main() {
    let username = get_username(10103);
    
    /*
    match username {
        Some(name) =>  println!("{}", name),
        None => {},
    }
    */

    // There is another syntax however
    // Since we only care about one variant
    // There is a better syntax than -> `None => {}`

    if let Some(name) = username {
        println!("{name}");
    }
}

fn get_username(uuid: i32) -> Option<String> {
    // get username from db
    if uuid == 10103 {
        let db_result = String::from("Billy");
            Some(db_result)
    } else {
        None
    }
}
