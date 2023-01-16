use std::format;
use petgraph::graph::Graph;
use petgraph::dot::Dot;
use std::convert::From;
use std::ops::Add;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Value<T, Y> //where T: From<f64>
{
    pub result : T,
    pub children : Vec<T>,
    pub op : Vec<char>,  // needs to be T
    pub grad : f64,
    pub label : Y      
}

impl <T: std::ops::Add<Output = T>, Y> Add for Value <T, Y> {
    type Output = Self;

    fn add(self, other: Self) -> Self
    {
        Self {
            result: self.result + other.result,
            children : self.children,
            op : self.op,
            grad : self.grad,
            label: self.label
        }
    }

}

impl <T: std::ops::Mul<Output = T>, Y> Mul for Value <T, Y> {
    type Output = Self;

    fn mul(self, other: Self) -> Self
    {
        Self {
            result: self.result * other.result,
            children : self.children,
            op : self.op,
            grad : self.grad,
            label: self.label
        }
    }

}




impl <T: std::fmt::Debug + Clone, Y: std::fmt::Debug + std::convert::From<String>> Value <T, Y>  where f64: From<T> + Copy{
    pub fn new(input: T, lab: Y) -> Self {
        Self{result : input,
            children : vec![],
            op : vec![],
            grad : 0.0,
            label: lab
        }
    }

    pub fn repr(&self) -> String {
        format!("Value(result={:?})", self)
    }

    pub fn add(&mut self, other: T)
    // TODO: implement this functionally
    where T: std::ops::Add<Output=T> + Copy // + From<char> // find a way to remove the copy trait
    {
        self.children.push(self.result);
        self.result = self.result + other;
        // new attempt 

        // Self{ 
        //     result : self.result + other,
        //     children : self.children.push(self.result),
        //     op: self.op.push('+'),
        //     label : self.label
        // };
            // this is confusing bc of reassignment, it needs to be more concise
            // self.prev.push(self.result);
            // self.op.push('+');//.into());  // for sign
            // self.children.push(self.result);

            // self.result = self.result + other;  // reassignment
            // self.children.push(other);
    }

    pub fn mul(& mut self, other: T)
    // again reimplement this functionally as we are being more verbose than necessary
    where T: std::ops::Mul<Output=T> + Copy  //+ From<char> // find a way to remove the copy trait

    {
        // self.prev.push(self.result);
        self.op.push('*');//.into()); // for sign 
        self.children.push(self.result);

        self.result = self.result * other;  // reassignment
        self.children.push(other);
    }

    pub fn grad(&mut self)
    where T: std::ops::Mul<Output=T> + std::ops::MulAssign + Copy       
    {

        self.grad += 1.0 * f64::from(self.result);
    }
    pub fn tanh(&mut self) -> Self 
    where T: std::ops::Mul<Output=T> + std::ops::MulAssign + Copy + From<f64>, Y: From<String> + Copy
    {
        let t = ((2.0 * f64::from(self.result)).exp() - 1.0) / 
            ((2.0 * f64::from(self.result)).exp() + 1.0);
        
        Self{result : t.into(),
            children : self.children.clone(),
            op : self.op.clone(),
            grad : self.grad,
            label: String::from("tanh").into()
        }
    }
    pub fn visualize(&self) {
        let mut graph = Graph::<&T, T>::new();
        graph.add_node(&self.result);

        println!("{:?}", Dot::new(&graph));

    }
}



