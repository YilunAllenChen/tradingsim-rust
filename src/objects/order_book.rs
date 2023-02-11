// use std::collections::BinaryHeap;
use std::collections::BTreeSet;
use std::fmt;

use crate::objects::Order;
use crate::objects::Side;

pub struct OrderBook {
    buys: BTreeSet<Order>,
    sells: BTreeSet<Order>
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            buys: BTreeSet::<Order>::new(),
            sells: BTreeSet::<Order>::new(),
        }
    }

    pub fn add(&mut self, order: Order) {
        if order.get_side() == Side::BUY {
            self.buys.insert(order);
        } else {
            self.sells.insert(order);
        }
    }
}

impl fmt::Display for OrderBook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Buys:\n{}\nSells:\n{}",
            self.buys
                .iter()
                .fold(String::new(), |acc, order| acc + &order.to_string() + "\n"),
            self.sells
                .iter()
                .fold(String::new(), |acc, order| acc + &order.to_string() + "\n")
        )
    }
}


// pub struct OrderBook {
//     buys: BinaryHeap<Order>,
//     sells: BinaryHeap<Order>,
// }

// impl OrderBook {
//     pub fn new() -> Self {
//         Self {
//             buys: BinaryHeap::<Order>::new(),
//             sells: BinaryHeap::<Order>::new(),

//         }
//     }

//     pub fn add(&mut self, order: Order) {
//         if order.get_side() == order::Side::BUY {
//             self.buys.push(order);
//         } else {
//             self.sells.push(order);
//         }
//     }

// }
// impl fmt::Display for OrderBook {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "Buys:\n{}\nSells:\n{}",
//             self.buys
//                 .iter()
//                 .fold(String::new(), |acc, order| acc + &order.to_string() + "\n"),
//             self.sells
//                 .iter()
//                 .fold(String::new(), |acc, order| acc + &order.to_string() + "\n")
//         )
//     }
// }