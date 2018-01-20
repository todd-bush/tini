extern crate tini;
use tini::Ini;

static INPUT: &'static str = "./examples/example.ini";

fn main() {
    let config = Ini::from_file(INPUT).unwrap();

    // get ints
    let int_v: Vec<u8> = config.get_vec("section_three", "frst1").unwrap();
    println!("frst1 = {:?}", int_v);

    // get floats
    let float_v: Vec<f32> = config.get_vec("section_three", "frst2").unwrap();
    println!("frst2 = {:?}", float_v);

    // get Strings
    // Warning
    //  data will be splitted by ',' character
    //  there is no exception
    let str_v: Vec<String> = config.get_vec("section_three", "frst3").unwrap();
    println!("frst3 = {:?}", str_v);

    // get bools
    let bool_v: Vec<bool> = config.get_vec("section_three", "frst4").unwrap();
    println!("frst4 = {:?}", bool_v);
}
