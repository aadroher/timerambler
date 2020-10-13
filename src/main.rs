mod models;

use models::Phase;
fn main() {
    let batxillerat = Phase {
        name: "Batxillerat".to_string(),
        code: "bat".to_string(),
    };
    println!("Hello, Timerambler!");
    println!("{:?}", batxillerat);
}
