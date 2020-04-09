extern crate libloading;


use libloading::{Library, Symbol};
use has_thread_local::{set_thread_local, get_thread_local};

fn main() {
    let lib = Library::new("libshared_library.so").unwrap();
    set_thread_local(10);
    unsafe {
        let func: Symbol<unsafe extern fn() -> u32> = lib.get(b"print_local").unwrap();
        func();
    };
    println!("From static executable:{}", get_thread_local());
}
