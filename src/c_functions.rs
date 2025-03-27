use std::ffi::c_void;




type CULONG = u64;
pub type pthread_t= CULONG;

#[repr(C)]
pub struct pthread_attr_t {}


#[link(name = "pthread")]
unsafe extern "C" {
    
    pub unsafe fn pthread_create(
        native: *mut pthread_t,
        attr: *const pthread_attr_t,
        f: extern "C" fn(*mut c_void) -> *mut c_void,
        value: *mut c_void,
    ) -> i32;

    pub unsafe fn pthread_join(val1: pthread_t, ret_type: *mut c_void) -> i32; 
}

