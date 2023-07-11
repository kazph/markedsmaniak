mod consumers;
mod market;
mod producers;
mod stats;


fn main() {


    let mut mar = market::Marked::create_empty();

    for count in 0..20 {
        mar.add_consumer(Box::new(consumers::SimplisticOnePurchaseConsumer::new(count * 10 + 45)));
    }

    for count in 0..15 {

        mar.add_producer(Box::new(producers::SimplisticOnePurchaseConsumer::new( count * 15 + 20)));
    }

    mar.shuffle();
    mar.run_round();

    let stats = mar.get_stats();

    let json_producer = serde_json::to_string_pretty(&stats.0).unwrap();
    let json_consumer = serde_json::to_string_pretty(&stats.1).unwrap();

    std::fs::write("output/producers.json", json_producer)
        .expect("Unable to write to file");
    std::fs::write("output/consumers.json", json_consumer)
        .expect("Unable to write to file");

    mar.reset();
}
