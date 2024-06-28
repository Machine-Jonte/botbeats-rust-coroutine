#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(stmt_expr_attributes)]
mod modules;
mod types;
mod usr_main;

//==== Main function
// Cannot change name. Host uses this to find the right function.
#[no_mangle]
pub extern "C" fn start() -> i32 {
    usr_main::main();
    88
}
