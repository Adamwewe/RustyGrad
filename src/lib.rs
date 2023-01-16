use std::format;
use petgraph::graph::Graph;
use petgraph::dot::Dot;
use std::convert::From;
use std::ops::Add;
use std::ops::Mul;

#[derive(Debug, Clone, Default)]
pub struct Value <T>
{
    pub result : f64,  // replace with vec to keep track of changes
    pub children : Vec<f64>,
    pub op : Vec<char>,  
    pub grad : f64,
    pub label : Vec<String>,
    pub backward : T
}

impl <T: std::default::Default + std::fmt::Debug> Value <T>{
    pub fn new(input: f64, lab: String) -> Self {
        Self{result : input,
            children : Vec::new(),
            op : Vec::new(),
            grad : 0.0,
            label: vec![lab],
            backward : Default::default()
        }
    }

    pub fn repr(&self) -> String {
        format!("Value(result={:?})", self)
    }

    pub fn add(&mut self, other: f64) {
        self.children.push(self.result);
        self.children.push(other);
        self.result = self.result + other;

        // verbosity to be fixed:

        self.op.push('+');

    }

    pub fn mul(& mut self, other: f64) {
        // self.prev.push(self.result);
        self.children.push(self.result);
        self.children.push(other);

        self.result = self.result * other;  // reassignment

        self.op.push('*'); // for sign 

    }

    pub fn grad(&mut self)
//     where T: std::ops::Mul<Output=T> + std::ops::MulAssign + Copy       
    {

        self.grad += 1.0 * f64::from(self.result);
    }
    pub fn tanh(&mut self) -> Self 
    {
        let t = ((2.0 * self.result).exp() - 1.0) / 
            ((2.0 * self.result).exp() + 1.0);

        // let formatter = ;
        self.label.push(String::from("tanh"));
        
        Self{result : t,
            children : self.children.clone(),
            op : self.op.clone(),
            grad : self.grad,
            label: self.label.clone(),
            backward : Default::default()
        }
    }
}

impl <T: std::default::Default> Add for Value<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self
    {
        Self {
            result: self.result + other.result,
            children : self.children,
            op : self.op,
            grad : self.grad,
            label: self.label,
            backward : Default::default()
        }
    }

}

impl <T: std::default::Default> Mul for Value <T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self
    {
        Self {
            result: self.result * other.result,
            children : self.children,
            op : self.op,
            grad : self.grad,
            label: self.label,
            backward : Default::default()
        }
    }
}






