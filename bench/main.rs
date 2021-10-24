use std::fs::read_to_string;
use std::time::Instant;

use rusty_source_map::consumer::{Consumer, ConsumerTrait, IterOrd};

fn main() {
    let content = read_to_string("./bench/angular-min-source-map.json").unwrap();
    let a = Instant::now();
    let consumer = rusty_source_map::consumer::create_consumer(&content).unwrap();
    if let Consumer::BasicConsumer(mut consumer) = consumer {
        consumer.each_mapping(|_| {}, IterOrd::GeneratedOrd)
    }
    let elapsed = a.elapsed();

    println!("{:?}", elapsed);
}
