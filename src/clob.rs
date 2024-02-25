use std::{
    cell::RefCell,
    collections::BTreeMap,
    ops::{Bound, Range},
};

use anyhow::{anyhow, Result};
use std::collections::btree_map::CursorMut;
use uuid::Uuid;

#[derive(Debug)]
pub struct Order {
    pub quantity: u64,
    pub max_price: u64,
    pub sender: Sender,
}

#[derive(Debug)]
pub struct Book {
    asks: RefCell<BTreeMap<u64, Vec<Order>>>,
    bids: RefCell<BTreeMap<u64, Vec<Order>>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Sender {
    pub id: Uuid,
    bal: u64,
}

impl Sender {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            bal: 0,
        }
    }

    pub fn mint(&mut self, by: u64) {
        self.bal += by;
    }

    pub fn burn(&mut self, by: u64) {
        self.bal -= by;
    }

    pub fn bal(&self) -> u64 {
        self.bal
    }
}

impl Book {
    pub fn new() -> Self {
        Self {
            asks: RefCell::new(BTreeMap::new()),
            bids: RefCell::new(BTreeMap::new()),
        }
    }

    pub fn ask(&self, max_price: u64, quantity: u64, sender: Sender) -> Result<()> {
        let mut order = Order {
            quantity,
            max_price,
            sender,
        };

        let qty_left = self.match_order(&mut order, false)?;

        // if can't fill completely
        if qty_left > 0 {
            if self.asks.borrow().contains_key(&max_price) {
                let mut asks_mut = self.asks.borrow_mut();
                let lo = asks_mut.get_mut(&max_price).unwrap();

                lo.push(order);
            } else {
                self.asks.borrow_mut().insert(max_price, vec![order]);
            }
        }

        // println!("{:#?}", self.asks.borrow());

        Ok(())
    }

    /// try to match the orders from current orderbook.
    fn match_order(&self, order: &mut Order, is_bid: bool) -> anyhow::Result<u64> {
        let mut opp_order_mut = if is_bid {
            self.asks.borrow_mut()
        } else {
            self.bids.borrow_mut()
        };

        for (op_order_price, op_orders) in opp_order_mut.range_mut(0..(order.max_price + 1)) {
            // println!(
            //     "op_order price, max price: {}, {}",
            //     op_order_price, &order.max_price
            // );
            assert!(op_order_price <= &order.max_price);

            if op_orders.is_empty() {
                continue;
            }

            op_orders.retain_mut(|op_order| {
                if op_order.quantity > order.quantity {
                    op_order.quantity -= order.quantity;
                    order.quantity = 0;
                    return true;
                } else {
                    // case: op_order.quantity <= order.quantity
                    order.quantity -= op_order.quantity;
                    return false;
                }
            });
        }

        Ok(order.quantity)
    }

    pub fn bid(&self, max_price: u64, quantity: u64, sender: Sender) -> Result<()> {
        let mut order = Order {
            quantity,
            max_price,
            sender,
        };

        let qty_left = self.match_order(&mut order, true)?;

        // if can't fill completely
        if qty_left > 0 {
            if self.bids.borrow().contains_key(&max_price) {
                let mut bids_mut = self.bids.borrow_mut();
                let lo = bids_mut.get_mut(&max_price).unwrap();

                lo.push(order);
            } else {
                self.bids.borrow_mut().insert(max_price, vec![order]);
            }
        }
        // println!("{:#?}", self.asks.borrow());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use super::*;
    use crate::clob::Book;
    use rand::Rng;

    #[test]
    fn test_vec() {
        let mut v = vec![12, 34, 435, 56];
        println!("{:?}", v);
        v.remove(1);
        println!("{:?}", v);
    }

    // #[bench]
    // fn benchmark_orderbook(b: &mut Bencher) {
    //     b.iter(|| {
    //         let mut book = Book::new();
    //         for i in 1..1000 {
    //             let sender = Sender::new();
    //             let mut r = rand::thread_rng();
    //             if r.gen_bool(1.0 / 3.0) {
    //                 // generate buy order
    //                 book.ask(r.gen_range(1..100), r.gen_range(10..1000), sender)
    //                     .unwrap();
    //             } else {
    //                 // generate sell order
    //                 book.ask(r.gen_range(1..100), r.gen_range(10..1000), sender)
    //                     .unwrap();
    //             }
    //         }
    //         // println!("{:?}", book);
    //     });
    // }

    #[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
    struct B {
        id: u64,
        b: u64,
    }
    impl B {
        pub fn n(id: u64, b: u64) -> Self {
            Self { id, b }
        }
    }

    #[test]
    fn test_btree_map_struct() -> anyhow::Result<()> {
        let mut bm: BTreeMap<B, String> = BTreeMap::new();
        bm.insert(B::n(1, 21), "something".to_string());
        bm.insert(B::n(2, 45), "something3".to_string());
        bm.insert(B::n(3, 20), "something5".to_string());
        bm.insert(B::n(4, 19), "something55".to_string());

        for i in bm.iter() {
            println!("{:?}", i);
        }
        Ok(())
    }

    #[test]
    fn test_btree_map() -> anyhow::Result<()> {
        let mut bm: BTreeMap<u64, String> = BTreeMap::new();
        bm.insert(10, "something".to_string());
        bm.insert(3, "something3".to_string());
        bm.insert(5, "something5".to_string());
        bm.insert(5, "something55".to_string());

        for i in bm.iter() {
            println!("{:?}", i);
        }
        Ok(())
    }

    #[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
    struct A {
        v: u64,
    }

    #[test]
    fn test_btree_set_struct() -> anyhow::Result<()> {
        let mut bm: BTreeSet<A> = BTreeSet::new();
        bm.insert(A { v: 45 });
        bm.insert(A { v: 43 });
        bm.insert(A { v: 41 });
        bm.insert(A { v: 49 });
        bm.insert(A { v: 49 });

        for i in bm.iter() {
            println!("{:?}", i);
        }
        Ok(())
    }

    #[test]
    fn test_btree_set() -> anyhow::Result<()> {
        let mut bm: BTreeSet<String> = BTreeSet::new();
        bm.insert("something".to_string());
        bm.insert("something3".to_string());
        bm.insert("something5".to_string());
        bm.insert("something55".to_string());

        for i in bm.iter() {
            println!("{:?}", i);
        }
        Ok(())
    }

    #[test]
    fn test_orderbook() -> anyhow::Result<()> {
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
}
