mod mthread;
mod c_functions;
use std::{thread, time::Duration};

use mthread::Mthread;

fn main() {
    // let mut a = Arc::new(Mutex::new(String::new()));
    // let data = Arc::clone(&a);
    // thread::spawn( move ||{
    //     let guard = a.lock()?;
    //     println!("{guard}");
    //     Ok(())
    //
    // });
   let mthread = Mthread::spawn(||{
        for  i in 0..100{
            println!("sus {i}");
        }
    });
    let mthread2 = Mthread::spawn(move ||{
        for j in 0..100{
            println!("Stan {j}");   
        }
    });

    mthread.join();
    mthread2.join();
}
// program is running concurently not parallely .?????

