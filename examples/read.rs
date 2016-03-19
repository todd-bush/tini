extern crate tini;
use tini::Ini;

static INPUT: &'static str = "./examples/example.ini";

fn main() {
    let config = Ini::from_file(INPUT).unwrap();

    // if you are sure
    let name1: i32 = config.get("section_one", "name1").unwrap();

    // if you aren't sure
    let mut name5: bool = false;
    name5 = config.get("section_one", "name5").unwrap_or(name5); // non-existing key

    // check
    println!("name1: {}", name1);
    println!("name5: {}", name5);
}
