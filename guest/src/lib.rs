mod generated;
extern crate wapc_guest as guest;
pub use generated::*;
use guest::prelude::*;

#[no_mangle]
pub fn wapc_init() {
    Handlers::register_say_hello(say_hello);
}

fn say_hello(name: String) -> HandlerResult<String> {
    Ok(format!("Hello there {}", name).to_string()) // TODO: Provide implementation.
}
