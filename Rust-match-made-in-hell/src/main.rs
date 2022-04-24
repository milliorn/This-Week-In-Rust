use rand::Rng;

fn is_good() -> bool {
    true
}

fn process(secure: bool) {
    if secure {
        println!("No hackers")
    } else {
        println!("Come on in")
    }
}

fn main() {
    let good = match is_good() {
        true => "It is good",
        false => "It isn't good, yet",
    };
    println!("{good}");

    let results = match rand::thread_rng().gen_range(0..=10) {
        10 => "Overwhelming victory",
        5.. => "Victory",
        _ => "Defeat",
    };
    println!("{results}")
}
