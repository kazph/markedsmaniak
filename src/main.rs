// use consumers::simplistic_one_purchace::SimplisticOnePurchaseConsumer;
use consumers::market_aware_one_purchace::MarketAwareOnePurchase;

// use producers::simplistic_one_product::SimplisticOneProductProducer;
use producers::market_aware_one_product::MarkedAwareOneProduct;

mod consumers;
mod market;
mod producers;
mod stats;

fn main() {
    let mut mar = market::Marked::create_empty();

    for count in 0..20 {
        mar.add_consumer(Box::new(MarketAwareOnePurchase::new(count * 5)));
        mar.add_producer(Box::new(MarkedAwareOneProduct::new(count * 5)));
    }

    for _ in 0..500 {
        mar.reset();
        mar.shuffle();
        mar.run_round();
    }


    let stats = mar.get_stats();

    let json_producer = serde_json::to_string_pretty(&stats.0).unwrap();
    let json_consumer = serde_json::to_string_pretty(&stats.1).unwrap();

    std::fs::write("output/producers.json", json_producer).expect("Unable to write to file");
    std::fs::write("output/consumers.json", json_consumer).expect("Unable to write to file");
}
