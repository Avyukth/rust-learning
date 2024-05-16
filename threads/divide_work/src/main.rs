use std::thread::{self, JoinHandle};

fn main() {
    const  N_THREADS:usize = 8;

    let to_add:Vec<u32> =  (0..8000).collect();
    let mut thread_handles:Vec<JoinHandle<u32>>  = Vec::new();
    let chunks = to_add.chunks(to_add.len()/N_THREADS);

    for chunk in chunks {
        let my_chunk = chunk.to_owned();
        thread_handles.push(thread::spawn(move || {
            my_chunk.iter().sum()
        }));
    }

    let mut sum = 0 ;
    for handle in thread_handles {
          sum+= handle.join().unwrap();
    }
    println!("sum is {}", sum);
}
