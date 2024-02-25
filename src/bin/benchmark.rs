use clob::clob::{Book, Sender};
use rand::Rng;
use std::time::SystemTime;

fn main() -> anyhow::Result<()> {
    let start = SystemTime::now();

    let book = Book::new();
    let mut r = rand::thread_rng();

    for i in 1..100000 {
        let sender = Sender::new();
        if r.gen_bool(1.0 / 3.0) {
            // generate buy order
            book.ask(r.gen_range(1..100), r.gen_range(10..1000), sender)
                .unwrap();
        } else {
            // generate sell order
            book.bid(r.gen_range(1..100), r.gen_range(10..1000), sender)
                .unwrap();
        }
    }

    let after = SystemTime::now().duration_since(start)?;

    println!("time: {:?}", after.as_nanos());
    println!("operation per seconds: {:?}", 1000.0 / after.as_secs_f64());
    Ok(())
}
