mod lib; 
use lib::Value;

fn main() {

    let mut variable : Value<f64> = Value::new(32.0, "a".into());


    let mut vals : Value<f64> = Value::new(32.0, "a".into()); 
    vals.add(4.0);
    vals.mul(2.0);
    // println!("{:?}", vals);

    vals.add(1.0);
    println!("{:?}", vals.repr());

    vals.add(2.0);
    println!("{:?}", vals.repr());

    vals.add(4.9);


    println!("{:?}", vals.repr());

    let mut vals2 :  Value<f64> = Value::new(20.0, "b".into());  
    println!("{:?}", vals.repr());
    vals.tanh();

    println!("{:?}", vals.repr());

    vals2.tanh();
    println!("{:?}", vals2.repr());
    // vals2.visualize();
}
