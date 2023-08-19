// you can make a module in
// it's own file like this, but go to
// lib.rs to see that it is imported into
// the module tree

// NOTE - File needs to have the same name as
// the module and not have sub-modules

pub enum Status {
    Connected,
    Interrupted,
}

pub fn connect_to_db() -> Status {
    return Status::Connected;
}

pub fn get_user() {
    // get user from database
}
