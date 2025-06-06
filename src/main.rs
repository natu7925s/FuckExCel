// main.rs
mod fec;
use fec::{FuckExCel, Value};

fn main() {
    let mut fec = FuckExCel::new();

    // 値セット
    fec.cell("A1").set(Value::Int(10));
    fec.cell("A1x2").set(Value::Formula("A1 * 2".to_string()));
    fec.cell("A1x3").set(Value::Formula("A1 * 3".to_string()));
    fec.cell("A1x4").set(Value::Formula("A1 * 4".to_string()));

    // 一旦表示
    println!("A1 = {:?}", fec.cell("A1").get());
    println!("A1x2 = {:?}", fec.cell("A1x2").get());
    println!("A1x3 = {:?}", fec.cell("A1x3").get());
    println!("A1x4 = {:?}", fec.cell("A1x4").get());

    // A1変更
    fec.cell("A1").set(Value::Int(20));

    // 表示
    println!("A1 = {:?}", fec.cell("A1").get());
    println!("A1x2 = {:?}", fec.cell("A1x2").get());
    println!("A1x3 = {:?}", fec.cell("A1x3").get());
    println!("A1x4 = {:?}", fec.cell("A1x4").get());
}
