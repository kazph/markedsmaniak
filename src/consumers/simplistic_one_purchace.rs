use crate::market::Consumer;
use crate::stats::ConsumerStats;

pub struct SimplisticOnePurchaseConsumer {
    max_price: i32,

    // -- STATS PURPOSES ---
    purchased: Option<i32>,
}

impl SimplisticOnePurchaseConsumer {
    pub fn new(max_price: i32) -> Self {
        return SimplisticOnePurchaseConsumer {
            max_price,
            purchased: None,
        };
    }
}

impl Consumer for SimplisticOnePurchaseConsumer {
    fn give_offer(&mut self, offer: i32) -> bool {
        if offer <= self.max_price && self.purchased.is_none() {
            self.purchased = Some(offer);
            return true;
        }

        return false;
    }

    fn reset(&mut self) {
        self.purchased = None;
    }

    fn get_stats(&self) -> ConsumerStats {
        ConsumerStats {
            max_price: self.max_price,
            purchase_price: self.purchased.unwrap_or(0),
            purchased: self.purchased.is_some(),
            expected_price: self.max_price,
        }
    }
}
