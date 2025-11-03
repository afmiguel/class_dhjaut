use std::rc::Rc;
use std::time::Duration;
use std::thread;

fn main() {
    let numbers = Rc::new(vec![1,2,3,4,5]);

    println!("Vetor: {:?}", numbers);

    thread::sleep(Duration::from_secs(1));
}
