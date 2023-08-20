#![allow(dead_code, unused_variables)]


// External dependencies (from crates.io) 
use rand::prelude::*;


// Note modules don't map to file structure
// They map to the module tree and multiple
// modules can be created in a file

// Since the modules doesn't have a sub module
// we can simply put in a file of the same name
// and bring it into scope like this...
pub mod database;

// auth_utils has sub-modules
//      so we defined a folder called auth_utils
//      Inside the folder we defined a mod.rs file
//      and a models.rs file (the sub-module)
//      We then import models mod into mod.rs
//      and then import auth_utils into this file
mod auth_utils;

// bringing Credentials into scope as `Credentials`

// We are also re-exporting the Credential modules with
// the pub keyword
pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenicator(creds: Credentials) {
    let timeout = 
        thread_rng().gen_range(100..500);
    println!("The Timeout is {}", timeout);
    if let Status::Connected = database::connect_to_db() {
        auth_utils::login(creds);
    }
}
