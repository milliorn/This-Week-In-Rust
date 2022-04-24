use rand::Rng;

pub enum Protection {
    Secure,
    #[deprecated = "using secure mode everywhere is now strongly recommended"]
    Insecure,
}

fn is_good() -> bool {
    true
}

fn process(prot: Protection) {
    match prot {
        Protection::Secure => {
            println!("No hackers")
        }
        #[allow(deprecated)]
        Protection::Insecure => {
            println!("Come in")
        }
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
    println!("{results}");

    process(Protection::Insecure)
}
