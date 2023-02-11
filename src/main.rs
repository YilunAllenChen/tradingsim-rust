mod objects;
use objects::MatchingEngineModule;
use objects::Order;
use objects::OrderBook;

use std::sync::mpsc;
use std::thread;
use std::time;


fn main() {
    let (tx, rx) = mpsc::channel();
    let matching_engine = MatchingEngineModule::new(rx);
    // let mut new_book = OrderBook::new();
    for i in 0..10 {
        let new_order = Order::new(i, 100.0 + i as f64, -51.0 + i as f64 * 10.0, i);
        // new_book.add(new_order);
        let _ = tx.send(new_order);
    }
    thread::sleep(time::Duration::from_secs(1));
    // println!("{}", new_book);
}
