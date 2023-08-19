// this is a better approach imho,
// but you can also have a `mod.rs` file in the
// auth_utils folder and put the code below into it
// and delete this file

// having a lot of mod.rs files seems none ideal however

pub fn login(creds: models::Credentials) {
    // authenticate
    crate::database::get_user();
}

fn logout() {
    // log user out..
}

pub mod models;
