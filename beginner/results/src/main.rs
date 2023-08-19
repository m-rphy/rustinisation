
//  The Result Enum 

/*
 * enum Result <T, E> {
 *      ok(T),
 *      Err(E),
 * }
 *
 */

// Representing a value that can Succeed or Fail -> Result enum
//
// Representung a valid valid or NO value -> Option enum


fn main() {
    
    let username = get_username(10103);
    if let Some(name) = username {
        println!("{name}");
    }
}


fn get_username(uuid: i32) -> Option<String> {

    let query = 
        format!("GET username FROM users WHERE id={uuid}");

    let db_result = query_db(query);
    
    if uuid == 10103 {
        db_result.ok()
    } else {
        None
    }
}


fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("Query is empty!"))
    } else {
        Ok(String::from("Billy"))
    }
}
