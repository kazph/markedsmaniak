use crate::producers;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Consumer {
    identifier: i32,
    max_price: i32,
    purchased: bool,
}

impl Consumer {
    pub fn new(id: i32, max_price: i32) -> Self {
        Self {
            identifier: id,
            max_price,
            purchased: false,
        }
    }

    pub fn merchent_offer(&mut self, producer: &mut producers::Producer) -> bool {
        if self.purchased == true {
            return false;
        }

        let accepted_offer = producer.offer(self.max_price);

        if accepted_offer {
            self.purchased = true;
        }
        return self.purchased;
    }

    pub fn get_status(&self) -> Self {
        return self.clone();
    }

    pub fn react_and_reset(&mut self) {
        self.max_price += if self.purchased { -5 } else { 5 };

        // reset
        self.purchased = false;
    }
}
