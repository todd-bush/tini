extern crate tiny_ini;
use tiny_ini::Ini;

fn main() {
    let config = Ini::from_file("../example.ini");
    let p1 = config.section("test");
    println!("p1 = {:?}", p1);
}