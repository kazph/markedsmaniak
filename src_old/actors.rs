pub trait ActorConsumer {
    fn merchent_offer(&mut self, producer: &mut Box<dyn ActorProducer>) -> bool;

    fn get_status(&self) ->  Box<dyn ActorConsumer>;

    fn react_and_reset(&mut self);
}

pub trait ActorProducer {
    fn offer(&mut self, price: i32) -> bool;

    fn get_status(&self) -> Self;

    fn react_and_reset(&mut self);
}
