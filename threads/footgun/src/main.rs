use std::{sync::atomic::AtomicI32, thread};

static  COUNTER:AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut handles = Vec::new();
    for _ in 0..1000{
        let handle = thread::spawn(|| {
            for _ in 0..100{
                COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("counter: {}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}
