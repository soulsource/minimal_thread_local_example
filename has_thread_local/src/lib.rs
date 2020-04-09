use std::cell::RefCell;
use std::ops::Deref;


thread_local! {
    pub static A_THREAD_LOCAL : RefCell<u64> =  RefCell::new(0);
}

pub fn set_thread_local(val: u64) {
    A_THREAD_LOCAL.with(|refcell| { refcell.replace(val); })
}

pub fn get_thread_local() -> u64 {
    A_THREAD_LOCAL.with(|refcell| *refcell.borrow().deref())
}
