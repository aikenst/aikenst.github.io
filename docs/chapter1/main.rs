use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..101);
    println!("{}",secret);
}