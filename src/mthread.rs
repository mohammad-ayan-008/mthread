use std::{
    ffi::c_void, ptr::{null, null_mut}
};
use rand::prelude::*;

use crate::c_functions::{pthread_t,pthread_join,pthread_create};


pub struct Mthread{
    id:pthread_t
}
impl Mthread {
    pub fn swap(task:impl  FnOnce() + Send + 'static)->Self{
        let mut num:Vec<i32> = (1..1000).collect();
        let mut rng = rand::rng();
        num.shuffle(&mut rng);
        let mut p:pthread_t =*num.choose(&mut rng).unwrap() as u64; 
        swap(&mut p as *mut pthread_t, Box::new(task));
        Self { id:p}
    }
    pub fn join(&self){
        join(self.id);
    }
}


fn swap(native: *mut pthread_t,task:Box<dyn FnOnce() +'static>){
    let fns = swapper;
    let ptr = Box::new(task);
    unsafe {
        pthread_create(native, null(), fns,Box::into_raw(ptr) as *mut c_void);
    }
}

fn join(val: pthread_t) {
    unsafe {
        pthread_join(val, null_mut());
    }
}

extern "C" fn swapper(t: *mut c_void) -> *mut c_void {

      let mut  k:*mut Box<dyn FnOnce()> = t.cast::<Box<dyn FnOnce() >>();
      let data = unsafe{Box::from_raw(k)};
      data();
      null_mut()
}


