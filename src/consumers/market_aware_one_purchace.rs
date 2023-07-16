use crate::market::Consumer;
use crate::stats::ConsumerStats;

#[derive(Debug)]
pub struct MarketAwareOnePurchase {
    max_price: i32,
    marked_expected_price: i32,

    // -- STATS PURPOSES ---
    purchased: Option<i32>,
}

impl MarketAwareOnePurchase {
    pub fn new(max_price: i32) -> Self {
        return MarketAwareOnePurchase {
            max_price,
            marked_expected_price: max_price,
            purchased: None,
        };
    }
}

impl Consumer for MarketAwareOnePurchase {
    fn give_offer(&mut self, offer: i32) -> bool {
        if offer <=  self.marked_expected_price && self.purchased.is_none() {
            self.purchased = Some(offer);
            return true;
        }

        return false;
    }

    fn reset(&mut self) {
        let purchased_product = self.purchased.is_some();
        
        if purchased_product {
            self.marked_expected_price -= 5;
        } else {
            self.marked_expected_price += 5;
        }

        self.marked_expected_price = std::cmp::min(self.max_price, self.marked_expected_price);
        self.purchased = None;
    }

    fn get_stats(&self) -> ConsumerStats {
        ConsumerStats {
            max_price: self.max_price,
            purchase_price: self.purchased.unwrap_or(0),
            purchased: self.purchased.is_some(),
            expected_price: self.marked_expected_price,
        }
    }
}
