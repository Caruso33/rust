#[derive(Debug)]
pub struct Bill {
    name: String,
    amount: f32,
}

impl Bill {
    pub fn new(name: String, amount: f32) -> Self {
        Bill { name, amount }
    }
}

pub enum Command {
    View,
    Add,
    Remove,
    Edit,
    Exit,
}