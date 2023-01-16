mod lib; 
use lib::Value;

fn main() {
    let mut vals : Value<f64, String> = Value::new(32.0, "a".into()); 
    vals.add(40.0);
    println!("{:?}", vals);

    vals.mul(20.0);
    println!("{:?}", vals);

    vals.add(200.0);


    let mut vals2 : Value<f64, String> = Value::new(20.0, "b".into()) + vals * Value::new(200.0, "c".into());   
    println!("{:?}", vals2);

    vals2.tanh();
    vals2.visualize();
}
