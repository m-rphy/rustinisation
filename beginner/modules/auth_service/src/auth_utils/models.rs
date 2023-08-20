

pub struct Credentials {
    username: String,
    password: String,
}

// We could make the fields of Credentials 
// public or we could make a publlic impl block
// with a pub new fn 

// I like this version more personally
impl Credentials {
   pub fn new(username: String, password: String) -> Self {
        Credentials {
            username,
            password,
        }
    }

}
