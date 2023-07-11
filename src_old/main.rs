mod actors;
mod consumers;
mod producers;

use consumers::*;
use producers::*;
use crate::actors::{ActorConsumer, ActorProducer};

use rand::seq::SliceRandom;
use rand::thread_rng;

struct Marked {
    consumers: Vec<Box<dyn ActorConsumer>>,
    producers: Vec<Box<dyn ActorProducer>>,
    
    consumer_data: Vec<Vec<Consumer>>,
    producer_data: Vec<Vec<Producer>>,
}

impl Marked {
    fn new() -> Self {
        Self {
            consumers: Vec::new(),
            producers: Vec::new(),
            consumer_data: vec![],
            producer_data: vec![],
        }
    }

    fn populate(&mut self) {
        for i in 0..10 {
            self.consumers.push(Consumer::new(i, 100));
            self.producers.push(Producer::new(i, 90));
        }

        self.producers.push(Producer::new(10, 90));
        self.producers.push(Producer::new(11, 90));
        self.producers.push(Producer::new(12, 90));
        self.producers.push(Producer::new(13, 90));
    }

    fn tick(&mut self) {
        for consumer in self.consumers.iter_mut() {
            for producer in self.producers.iter_mut() {
                consumer.merchent_offer(producer);
            }
        }
    }

    fn status(&mut self) {
        let consumer_status: Vec<Consumer> =
            self.consumers.iter().map(|c| c.get_status()).collect();
        let producer_status: Vec<Producer> =
            self.producers.iter().map(|c| c.get_status()).collect();

        self.consumer_data.push(consumer_status);
        self.producer_data.push(producer_status);
    }

    fn react(&mut self) {
        let mut rng = thread_rng();

        for consumer in self.consumers.iter_mut() {
            consumer.react_and_reset();
        }

        for producer in self.producers.iter_mut() {
            producer.react_and_reset();
        }

        self.consumers.shuffle(&mut rng);
        self.producers.shuffle(&mut rng);
    }

    fn save(&self) {
        let json_producer_data = serde_json::to_string_pretty(&self.producer_data).unwrap();
        let json_consumer_data = serde_json::to_string_pretty(&self.consumer_data).unwrap();

        std::fs::write("output/producers.json", json_producer_data)
            .expect("Unable to write to file");
        std::fs::write("output/consumers.json", json_consumer_data)
            .expect("Unable to write to file");
    }
}

fn main() {
    let mut marked = Marked::new();
    marked.populate();

    for itr in 0..11 {
        println!("Iteration {itr}");
        marked.tick();

        marked.status();

        marked.react();
    }

    marked.save();
}
