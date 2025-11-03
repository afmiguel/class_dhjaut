use std::sync::Arc;
use std::time::Duration;
use std::thread;

fn main() {
    let numbers = Arc::new(vec![1,2,3,4,5]);

    for _ in 0..2{
        let numbers_clone = Arc::clone(&numbers);

        thread::spawn(move || {
            println!("Vetor: {:?}", numbers_clone);
        });
    }

    thread::sleep(Duration::from_secs(1));
}
