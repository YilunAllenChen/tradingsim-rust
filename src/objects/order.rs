use std::cell::Cell;
use std::fmt;

pub struct Order {
    oid: u64,
    price: f64,
    quantity: f64,
    quantity_remaining: f64,
    timestamp_ms: u64,
    side: Side,
}
#[derive(PartialEq, Clone, Copy)]
pub enum Side {
    BUY,
    SELL,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Side::BUY => write!(f, "BUY "),
            Side::SELL => write!(f, "SELL"),
        }
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "oid: {} | {} | px: {} | qty: {}/{}",
            self.oid,
            self.side,
            self.price,
            self.quantity_remaining.abs(),
            self.quantity.abs(),
        )
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.side, other.side) {
            (Side::BUY, Side::SELL) | (Side::SELL, Side::BUY) => {
                panic!("Can't compare buy with sell order")
            }
            (Side::BUY, Side::BUY) => self.price.partial_cmp(&other.price).unwrap(),
            (Side::SELL, Side::SELL) => other.price.partial_cmp(&self.price).unwrap(),
        }
    }
}

impl Eq for Order {}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price && self.timestamp_ms == other.timestamp_ms
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Order {
    pub fn new(oid: u64, price: f64, quantity: f64, timestamp_ms: u64) -> Self {
        if quantity == 0.0 {
            panic!("Can't create order with quantity 0")
        }
        Self {
            oid: oid,
            price: price,
            quantity: quantity,
            quantity_remaining: quantity,
            timestamp_ms: timestamp_ms,
            side: if quantity > 0.0 {
                Side::BUY
            } else {
                Side::SELL
            },
        }
    }

    pub fn get_side(&self) -> Side {
        self.side
    }

    pub fn set_remaining_quantity(&mut self, new_quantity: f64) {
        if self.quantity * new_quantity < 0.0 {
            panic!("new quantity can't be of a differnt sign")
        }
        self.quantity_remaining = new_quantity;
    }
}
