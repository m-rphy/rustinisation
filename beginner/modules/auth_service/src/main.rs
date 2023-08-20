
// demoing how to use the auth_service 
// library crate in this binary crate

use auth_service::Credentials;

fn main() {
    
    let cred = Credentials::new( 
        "wjm_iii".to_owned(),
        "sercure_password".to_owned()
    );

    // Since the fields are private, you can't see them though! So this causes
    // an error. If you want to use the fields, they must be public
    
    // println!("username is {} and password is ", cred.username, cred.password);
} 
