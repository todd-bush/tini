extern crate tiny_ini;
use tiny_ini::{Ini, IniBuilder};

fn main() {
    let config = Ini::from_file("./example.ini");
    println!("config = {:?}", config);
    let n1: u32 = config.get_def("section_one", "name1", 0);
    println!("[section_one][name1] = {}", n1);

    let test = IniBuilder::new().section("section_one")
                                .item("a", "1")
                                .item("b", "2")
                                .section("section_two")
                                .item("c", "3")
                                .item("d", "4")
                                .build();
    println!("test = {:?}", test);
    test.to_file("./test.ini");
}
