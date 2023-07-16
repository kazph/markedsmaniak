use crate::stats::{ConsumerStats, ProducerStats};
use rand::{seq::SliceRandom, thread_rng};

pub trait Consumer {
    fn give_offer(&mut self, offer: i32) -> bool;

    fn reset(&mut self);

    fn get_stats(&self) -> ConsumerStats;
}

pub trait Producer {
    fn get_offer(&self) -> Option<i32>;

    fn accept_offer(&mut self, offer: i32);

    fn reset(&mut self);

    fn get_stats(&self) -> ProducerStats;
}

pub struct Marked {
    consumers: Vec<Box<dyn Consumer>>,
    producers: Vec<Box<dyn Producer>>,

    rng: rand::rngs::ThreadRng,
}

impl Marked {
    pub fn create_empty() -> Self {
        return Marked {
            consumers: vec![],
            producers: vec![],
            rng: thread_rng(),
        };
    }

    pub fn run_round(&mut self) {
        for prod in self.producers.iter_mut() {
            for consum in self.consumers.iter_mut() {
                let offer = prod.as_ref().get_offer().unwrap_or(i32::MAX);

                let accepted = consum.as_mut().give_offer(offer);

                if accepted {
                    println!("TRADE!");
                    prod.as_mut().accept_offer(offer);
                }
            }
        }
    }

    pub fn reset(&mut self) {
        for consum in self.consumers.iter_mut() {
            consum.reset();
        }

        for produce in self.producers.iter_mut() {
            produce.reset();
        }
    }

    pub fn add_consumer(&mut self, consumer: Box<dyn Consumer>) {
        self.consumers.push(consumer);
    }

    pub fn add_producer(&mut self, producer: Box<dyn Producer>) {
        self.producers.push(producer);
    }

    pub fn get_stats(&self) -> (Vec<ProducerStats>, Vec<ConsumerStats>) {
        let mut producer_stats = vec![];
        let mut consumer_stats = vec![];

        for prod in &self.producers {
            producer_stats.push(prod.get_stats());
        }

        for con in &self.consumers {
            consumer_stats.push(con.get_stats());
        }

        return (producer_stats, consumer_stats);
    }

    pub fn shuffle(&mut self) {
        self.consumers.shuffle(&mut self.rng);
        self.producers.shuffle(&mut self.rng);
    }
}
