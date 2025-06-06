mod fec;
use fec::FuckExCel;

fn main() {
    
    let mut fec = FuckExCel::new();

    fec.cell("A1").set(30);

    let val = fec.cell("A1").get();
    
    match val {
        Some(v) => println!("A1 = {}", v),
        None => println!("None"),
    }

}
