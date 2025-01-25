use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
struct Point {
    x: i32,
    y:i32,
}

fn main() {
    let point = Point {
        x: 3,
        y: 4,
    };

    let serialize = serde_json::to_string(&point).expect("Error");
    println!("{}",serialize);

    let deserialize: Point = serde_json::from_str(&serialize).expect("Error");
    println!("{:?}",deserialize)

}


