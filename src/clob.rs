use std::{cell::RefCell, collections::BTreeMap, ops::Bound};

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
            if let Some(lo) = self.asks.borrow_mut().get_mut(&max_price) {
                lo.push(order);
            } else {
                self.asks.borrow_mut().insert(max_price, vec![order]);
            }
        }

        Ok(())
    }

    fn match_order(&self, order: &mut Order, is_bid: bool) -> anyhow::Result<u64> {
        // try to match the orders from current orderbook.

        let cursor;
        if is_bid {
            cursor = self
                .asks
                .borrow_mut()
                .upper_bound_mut(Bound::Included(&order.max_price));
        } else {
            cursor = self
                .bids
                .borrow_mut()
                .upper_bound_mut(Bound::Included(&order.max_price));
        }

        while let (op_order_price, op_orders) = cursor.value_mut() {}

        // let len;
        // if is_bid {
        //     len = self.asks.borrow().len();
        // } else {
        //     len = self.bids.borrow().len();
        // }

        // let mut i = 0;
        // while i < len && order.quantity > 0 {
        //     let mut opp_orders_mut;
        //     if is_bid {
        //         opp_orders_mut = self.asks.borrow_mut();
        //     } else {
        //         opp_orders_mut = self.bids.borrow_mut();
        //     };

        //     let opp_order = opp_orders_mut.get_mut(i);
        //     if let None = opp_order {
        //         break;
        //     }
        //     let opp_order = opp_order.unwrap();

        //     if opp_order.max_price > order.max_price {
        //         i += 1;
        //         continue;
        //     }

        //     if opp_order.quantity == order.quantity {
        //         order.quantity = 0;
        //         opp_orders_mut.remove(i);
        //     } else if opp_order.quantity > order.quantity {
        //         opp_order.quantity -= order.quantity;
        //         order.quantity = 0;
        //         i += 1;
        //     } else if opp_order.quantity < order.quantity {
        //         order.quantity -= opp_order.quantity;
        //         opp_orders_mut.remove(i);
        //     }
        // }

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
            if let Some(lo) = self.bids.borrow_mut().get_mut(&max_price) {
                lo.push(order);
            } else {
                self.bids.borrow_mut().insert(max_price, vec![order]);
            }
        }

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
