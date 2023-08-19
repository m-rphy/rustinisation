#![allow(dead_code, unused_variables)]

// NOTE - Look into lib.rs for a better file structure

// This single file is mixing concerns
// and has servel layers of abstraction

// we should encapsulate db code and authenicator code
// into seperate modules

// Concerned with authentication
// Credentials of a user

// Because we are passing this into a public
// function, we need to make this data
// public as well
pub struct Credentials {
    username: String,
    password: String,
}

// Concern with db connection
// status of db connection
enum Status {
    Connected,
    Interrupted,
}

// Concern with db connection
fn connect_to_db() -> Status {
    return Status::Connected;
}

// Concerned with authentication
fn login(creds: Credentials) {
    // authenticate
    get_user();
}

// Concerned with authentication
fn logout() {
    // log user out..
}

// Concern with db connection
fn get_user() {
    // get user from database
}

// we want to expose this to consumers of this
// library

// Concerned with authentication
pub fn authenicator(creds: Credentials) {
    if let Status::Connected = connect_to_db() {
        login(creds);
    }
}
