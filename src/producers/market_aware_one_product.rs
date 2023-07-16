use crate::{market::Producer, stats::ProducerStats};

pub struct MarkedAwareOneProduct {
    min_price: i32,
    marked_expected_price: i32,

    sold: Option<i32>,
}

impl MarkedAwareOneProduct {
    pub fn new(min_price: i32) -> Self {
        MarkedAwareOneProduct {
            min_price,
            marked_expected_price: min_price,
            sold: None,
        }
    }
}

impl Producer for MarkedAwareOneProduct {
    fn get_offer(&self) -> Option<i32> {
        if self.sold.is_none() {
            return Some(self.marked_expected_price);
        }

        return None;
    }

    fn accept_offer(&mut self, offer: i32) {
        self.sold = Some(offer);
    }

    fn reset(&mut self) {
        let sold_product = self.sold.is_some();        

        if sold_product {
            self.marked_expected_price += 5;
        } else {
            self.marked_expected_price -= 5;
        }

        self.marked_expected_price = std::cmp::max(self.min_price, self.marked_expected_price);
        self.sold = None;
    }

    fn get_stats(&self) -> ProducerStats {
        ProducerStats {
            min_price: self.min_price,
            selling_price: self.sold.unwrap_or(0),
            sold: self.sold.is_some(),
            expected_price: self.marked_expected_price,
        }
    }
}
