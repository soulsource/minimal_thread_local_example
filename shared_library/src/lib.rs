use has_thread_local::get_thread_local;

#[no_mangle]
unsafe extern "system" fn print_local() {
    println!("From shared library:{}",get_thread_local());
}
