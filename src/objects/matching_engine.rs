use std::sync::mpsc;
use std::thread;

use crate::objects::Order;
use crate::objects::OrderBook;

pub struct MatchingEngineModule {
    receiver_handle: std::thread::JoinHandle<()>,
}

impl MatchingEngineModule {
    pub fn new(receiver: mpsc::Receiver<Order>) -> Self {
        let mut order_book = OrderBook::new();
        let receiver_handle = thread::spawn(move || loop {
            match receiver.recv() {
                Ok(order) => {
                    order_book.add(order);
                    println!("{order_book}");
                }
                Err(err) => {
                    println!("Error on receiver: {err}");
                    break;
                }
            }
        });
        Self {
            receiver_handle: receiver_handle,
        }
    }
    pub fn is_dead(&self) -> bool {self.receiver_handle.is_finished()}
}
