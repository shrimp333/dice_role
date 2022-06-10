extern crate rand;

use rand::thread_rng;
use rand::Rng;

pub fn run() {
    let mut rng = thread_rng();
    let dice: u8 = rng.gen_range(1..7);
    println!("Dice rolled was {}", dice);
}
