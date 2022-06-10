mod roll;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let x = args[1].parse().unwrap();
    
    let mut count = 0;
    while count < x {
        roll::run();
        count += 1;    
    }
}
