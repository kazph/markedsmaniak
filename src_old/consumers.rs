use crate::actors;
use crate::actors::ActorProducer;
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
}

impl actors::ActorConsumer for Consumer {
    fn merchent_offer(&mut self, producer: &mut impl ActorProducer) -> bool {
        if self.purchased == true {
            return false;
        }

        let accepted_offer = producer.offer(self.max_price);

        if accepted_offer {
            self.purchased = true;
        }
        return self.purchased;
    }

    fn get_status(&self) -> Self {
        return self.clone();
    }

    fn react_and_reset(&mut self) {
        self.max_price += if self.purchased { -5 } else { 5 };

        // reset
        self.purchased = false;
    }
}
