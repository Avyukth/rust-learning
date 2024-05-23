use std::{sync::Mutex, thread};

static MY_SHARED: Mutex<u32> = Mutex::new(30);

fn poisoner(){
    let mut lock = MY_SHARED.lock().unwrap();
    *lock+=1;
    panic!("THe poison strikes");
}


fn main() {
    
    // let my_shared = Mutex::new(0);
    // {
    //     let lock = MY_SHARED.lock().unwrap();
    //     drop(lock);
    //     if let Ok(_lock) = MY_SHARED.try_lock(){
    //         println!("got the lock");
    //     }else {
    //         println!("NO lock");
    //     } 

    // }


        let handle = thread::spawn(poisoner);
        println!("Trying to return from the thread");
        println!("{:?}", handle.join());

        let lock = MY_SHARED.lock();
        println!("{lock:?}");

        let recovered_data =  lock.unwrap_or_else(|poisoned|{
        println!("MUtex is poisoned , recovering data ...");
        poisoned.into_inner()
        });
        println!("Recovered data: {recovered_data:?}");
    }
