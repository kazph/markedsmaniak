use serde::Serialize;

#[derive(Serialize)]
pub struct ConsumerStats {
    pub purchased: bool,
    pub max_price: i32,
    pub purchase_price: i32,
    pub expected_price: i32,
}

#[derive(Serialize)]
pub struct ProducerStats {
    pub sold: bool,
    pub selling_price: i32,
    pub min_price: i32,
    pub expected_price: i32,
}
