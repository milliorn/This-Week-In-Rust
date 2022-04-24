fn is_good() -> bool {
    true
}

fn main() {
    let msg = match is_good() {
        true => "It is good",
        false => "It isn't good, yet",
    };
    print!("{msg}")
}
