mod models;

use models::Phase;
fn main() {
    let batxillerat = Phase {
        name: String::from("Batxillerat"),
        code: String::from("bat"),
    };
    println!("Hello, world!");
    println!("{:?}", batxillerat);
}
