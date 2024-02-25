// let's write a simple order matching algo.
#![feature(test)]

use clob::clob::{Book, Sender};
extern crate test;

fn main() -> anyhow::Result<()> {
    let book = Book::new();

    book.ask(120, 10, Sender::new())?;
    book.ask(110, 9, Sender::new())?;

    book.bid(110, 10, Sender::new())?;

    // for i in 1..5 {
    //     let sender = Sender::new();
    //     let mut r = rand::thread_rng();
    //     if r.gen_range(0..2) > 0 {
    //         // generate buy order
    //         book.ask(r.gen_range(1..10), r.gen_range(10..1000), sender)
    //             .unwrap();
    //     } else {
    //         // generate sell order
    //         book.bid(r.gen_range(1..10), r.gen_range(10..1000), sender)
    //             .unwrap();
    //     }
    // }
    println!("{:#?}", book);
    Ok(())
}
