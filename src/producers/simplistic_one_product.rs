use crate::{market::Producer, stats::ProducerStats};

pub struct SimplisticOneProductProducer {
    min_price: i32,
    sold: Option<i32>,
}

impl SimplisticOneProductProducer {
    pub fn new(min_price: i32) -> Self {
        SimplisticOneProductProducer {
            min_price,
            sold: None,
        }
    }
}

impl Producer for SimplisticOneProductProducer {
    fn get_offer(&self) -> Option<i32> {
        if self.sold.is_none() {
            return Some(self.min_price);
        }

        return None;
    }

    fn accept_offer(&mut self, offer: i32) {
        self.sold = Some(offer);
    }

    fn reset(&mut self) {
        self.sold = None;
    }

    fn get_stats(&self) -> ProducerStats {
        ProducerStats {
            min_price: self.min_price,
            selling_price: self.sold.unwrap_or(0),
            sold: self.sold.is_some(),
            expected_price: self.min_price,
        }
    }
}
