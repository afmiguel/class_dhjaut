use std::sync::Arc;
use std::time::Duration;
use std::thread;

fn main() {
    let numbers = Arc::new(vec![1,2,3,4,5]);

    thread::spawn(move || {
        println!("Vetor: {:?}", numbers);
    });

    thread::sleep(Duration::from_secs(1));
}
