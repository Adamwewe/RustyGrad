mod lib; 
use lib::Value;

fn main() {

    let mut variable : Value = Value::new(32.0, "a".into(), Some(1.0));


    let mut vals : Value = Value::new(32.0, "a".into(), Some(2.0)); 
    vals.add(&mut Value::new(4.0, "b".into(), Some(4.0)));
    vals.mul(&mut Value::new(4.0, "c".into(), Some(5.0)));
    println!("{:?}", vals);

    // vals.add(Value::new(45.0, "d".into()));
    // println!("{:?}", vals.repr());

    // vals.mul(Value::new(2.0, "e".into()));
    // println!("{:?}", vals.repr());

    // vals.add(Value::new(1.0, "f".into()));


    // println!("{:?}", vals.repr());

    // let mut vals2 :  Value<f64> = Value::new(20.0, "b".into());  
    // println!("{:?}", vals.repr());
    // vals.tanh();

    // println!("{:?}", vals.repr());

    // vals2.tanh();
    // println!("{:?}", vals2.repr());
    // vals2.visualize();
}
