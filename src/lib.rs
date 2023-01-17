use std::format;
// use petgraph::graph::Graph;
// use petgraph::dot::Dot;
use std::convert::From;
use std::ops::Add;
use std::ops::Mul;
use std::collections::HashSet;
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};
use std::convert::Into;


#[derive(Clone, Debug, Default)]
pub struct Value
    // where F: Fn(usize) -> usize
{
    pub result : f64,  // replace with vec to keep track of changes
    pub children : Vec<f64>,
    pub op : Vec<String>,  
    pub grad : f64,
    pub label : Vec<String>,
    pub _backward : () //Box<dyn FnOnce()>
}

#[derive(Debug, Clone)]
struct FloatWrapper(f64);

// impl <F: std::default::Default + std::fmt::Debug + Copy> Value <F> 
    // where F: std::ops::Fn(usize) -> usize {
    impl Value {
        pub fn new(input: f64, lab: String, gradient: Option<f64>) -> Self {
                
        Self{result : input,
            children : Vec::new(),
            op : Vec::new(),
            grad : match gradient {
                Some(x) => x,
                None => 0.0
            },
            label: vec![lab],
            _backward : Default::default()
        }
    }  

    pub fn repr(&self) -> String {
        format!("Value(result={:?})", self)
    }

    pub fn add(&mut self, other: & mut Value) //<F>)//, func: &dyn Fn()) 
        // where F: std::ops::Fn(usize) -> usize 
    {
        self.children.push(self.result);
        self.children.push(other.result);
        self.result = self.result + other.result;

        // verbosity to be fixed:
        self.op.push(String::from("+")); 

        let mut f = (|| {
        self.grad += 1.0 * self.grad;
        other.grad += 1.0 * self.grad;
        })();  // test this further
        
        // self._backward = f;


        // let _f = (|| {
        //     println!("inside!")
        // })();
    }

    pub fn mul(& mut self, other: &mut Value) { //<F>)
        // where F: std::ops::Fn(usize) //usize {
        // self.prev.push(self.result);
        self.children.push(self.result);
        self.children.push(other.result);

        self.result = self.result * other.result;  // reassignment

        self.op.push(String::from("*")); // for sign 

        // let mut f = (|| {

        self.grad = self.result * self.grad;
        other.grad = self.result * self.grad;
        
        // })();  // test this further
        println!("{:?}, {:?}", self.grad, self.result);    
        // self._backward = f;

    }

    pub fn grader(&mut self) {

        self.grad += 1.0 * f64::from(self.result);
    }


    // todo: functional implmentation of backprop instead of locally scoping
    // pub fn backward(&mut self, func: impl Fn(usize) -> usize + 'static){

    //     // self.grad = 1.0 * self 

    // }


    pub fn tanh(&mut self) -> Self 
    {
        let t = ((2.0 * self.result).exp() - 1.0) / 
            ((2.0 * self.result).exp() + 1.0);

        // let formatter = ;
        self.label.push(String::from("tanh"));
        self.op.push(String::from("tanh"));

        let mut f = (|| {
            self.grad = (1.0 - f64::powf(t, 2.0)) * self.grad;
            })();
        
        Self{result : t,
            children : self.children.clone(),
            op : self.op.clone(),
            grad : self.grad,
            label: self.label.clone(),
            _backward : self._backward
        }
    }

    pub fn backward(&mut self) {

        // lots of cloning and shitty coding here, redo later!!!!
        
        let mut topo = Vec::new();
        let mut visited  = HashSet::new();
   
        
        let mut topo_builder = |x: FloatWrapper|{
            if !visited.contains(&x) {
                visited.insert(x.clone());
                topo.push(x.clone());
            }
        };

        // recursive closure call 
        self.children.iter()
            .for_each(|x| 
            topo_builder(FloatWrapper(*x)));
    
        self.grad = 1.0;

        let mut topo = topo.iter()
                .map(|x| x.0)
                .collect::<Vec<f64>>();

        
        topo.reverse();


        println!("{:?}", topo);

        // topo.reverse();
        // println!("{:?}",topo);
        
        // .iter()
                // .map()

        


    }
}

// impl <F: std::default::Default + std::ops::Fn(usize) -> usize> Add for Value<F> {
impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            result: self.result + other.result,
            children : self.children,
            op : self.op,
            grad : self.grad,
            label: self.label,
            _backward : self._backward
        }
    }
}

// impl <F: std::default::Default + std::ops::Fn(usize) -> usize> Mul for Value<F> {
    impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self
    {
        Self {
            result: self.result * other.result,
            children : self.children,
            op : self.op,
            grad : self.grad,
            label: self.label,
            _backward : self._backward
        }
    }
}



// impl Hash for Value{
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         let bits = self.result.to_bits();
//         let bytes = unsafe {
//             std::mem::transmute::<u64, [u8; 8]>(bits)
//         };
//         state.write(&bytes);
//     }
// }

// impl PartialEq for Value {
//     fn eq(&self, other: &Self) -> bool {
//         (self.result - other.result).abs() < std::f64::EPSILON
//     }
// }


impl Hash for FloatWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let bits = self.0.to_bits();
        let bytes = unsafe { std::mem::transmute::<u64, [u8; 8]>(bits) };
        state.write(&bytes);
    }
}

impl PartialEq for FloatWrapper {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < std::f64::EPSILON
    }
}

impl Eq for FloatWrapper {}

// impl Eq for Value {}




