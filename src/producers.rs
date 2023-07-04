use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Producer {
    identifier: i32,
    min_price: i32,
    sold: bool,
}

impl Producer {
    pub fn new(id: i32, min_price: i32) -> Self {
        Self {
            identifier: id,
            min_price,
            sold: false,
        }
    }

    pub fn offer(&mut self, price: i32) -> bool {
        if self.sold == true {
            return false;
        }

        if price >= self.min_price {
            self.sold = true;
            return true;
        }
        return false;
    }

    pub fn get_status(&self) -> Self {
        return self.clone();
    }

    pub fn react_and_reset(&mut self) {
        self.min_price += if self.sold { 5 } else { -5 };

        // reset
        self.sold = false;
    }
}
