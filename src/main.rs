#![allow(unused)]
use std::fmt;
use std::cmp;
use rand::Rng;

type ArrayFunc = fn(f32) -> f32;

// defining Array struct, this uses Vec datatype (as in vector in c++) for dynamic memory allocation
#[derive(Clone)]
struct Array{
    data: Vec<f32>,
    shape: Vec<usize>,
    strides: Vec<usize>,
    backstrides: Vec<usize>,
    ndim: usize,
    itemsize: usize,
    totalsize: usize,
    c_order: bool,
    f_order: bool,
}

impl Array{
    fn new(shape: &[usize]) -> Self{
        let ndim = shape.len();
        let mut arr = Array{
            data : Vec::new(),
            shape: shape.to_vec(),
            strides: vec![0; ndim],
            backstrides: vec![0; ndim],
            ndim,
            itemsize: std::mem::size_of::<f32>(),
            totalsize: shape.iter().product(),
            c_order: false,
            f_order: false,
        };
        arr.recalculate_strides();
        arr.recalculate_backstrides();
        arr.set_array_flags();

        arr.data = vec![0.0; arr.totalsize];

        // return value below
        arr
    }

    fn set_array_flags(&mut self){
        self.c_order = self.strides[self.ndim-1] == self.itemsize;
        self.f_order = self.strides[0] == self.itemsize;
    }

    pub fn shape_as_vec(&self) -> Vec<usize> {
        self.shape.clone()
    }

    fn recalculate_strides(&mut self){
        self.strides[self.ndim-1] = self.itemsize;
        for i in (0..self.ndim-1).rev(){
            self.strides[i] = self.strides[i+1] * self.shape[i+1];
        }
    }

    fn recalculate_backstrides(&mut self){
        for i in (0..self.ndim).rev() {
            self.backstrides[i] = self.strides[i] * (self.shape[i]-1);
        }
    }

    fn from_values(&mut self, values: &[f32]){
        assert_eq!(self.totalsize, values.len());
        self.data.copy_from_slice(values);
    }

    fn random(shape: &[usize]) -> Self{
        let mut arr = Array::new(shape);
        let mut rng = rand::thread_rng();

        for i in 0..arr.totalsize {
            arr.data[i] = rng.gen_range(0.0..1.0);
        }
        arr
    }

    fn arrange(start: f32, end:f32, step: f32) -> Self{
        assert!(start<end && step>0.0);
        let len = ((end-start) / step).ceil() as usize;
        let mut arr = Array::new(&[len]);

        for i in 0..len{
            arr.data[i] = start + (i as f32) * step;
        }

        arr
        
    }

    fn print_info(&self){
        println!("Shape: {:?}", self.shape);
        println!("Strides: {:?}", self.strides);
        println!("Array is C-contiguous? {}", self.c_order);
        println!("Array is F-contiguous? {}", self.f_order);
    }

    fn traverse_helper(&self, curr: &mut usize, depth: usize){
        if depth==self.ndim-1{
            for i in 0..self.shape[self.ndim-1]{
                print!("{:.3}", self.data[*curr]);
                *curr += self.strides[self.ndim-1] / self.itemsize;
            }
            *curr += self.backstrides[self.ndim-1] / self.itemsize;

            println!();
            return;
        }
        self.traverse_helper(curr, depth+1);        // recursive call

        for _ in 1..self.shape[depth]{
            *curr += self.strides[depth]/self.itemsize;
            self.traverse_helper(curr, depth+1);
        }
        *curr += self.backstrides[depth]/self.itemsize;

        if depth!=0{
            println!();
        }
        
    }
    fn show(&self){
        self.traverse_helper(&mut 0, 0);
    }

    fn check_shape_equal(&self, other: &Array) -> bool{
        return self.ndim == other.ndim && self.shape == other.shape;
    }

    fn final_shape(&self, other: &Array) -> Option<Vec<usize>>{
        let res_ndim = cmp::max(self.ndim, other.ndim);     // resultant dimension
        let mut res_shape = vec![0; res_ndim];              // resultant shape
        for i in 0..res_ndim{
            let a = self.shape.get(i + self.ndim - res_ndim).copied().unwrap_or(1);
            let b = other.shape.get(i + other.ndim - res_ndim).copied().unwrap_or(1);

            if a==1 || b==1 || a==b{
                res_shape[i] = cmp::max(a, b);
            }
            else{
                return None;
            }
        }
        return Some(res_shape);
    }

    fn broadcast(&self, shape: &[usize]) -> Array{
        let mut res = Array::new(shape);

        if res.totalsize == self.totalsize{
            res.data.copy_from_slice(&self.data);
        }
        else{
            for i in 0..res.totalsize{
                res.data[i] = self.data[i % self.totalsize];
            }
        }

        return res;
    }

    fn reshape(&mut self, new_shape:&[usize]){
        assert_eq!(self.totalsize, new_shape.iter().product());

        self.ndim = new_shape.len();
        self.shape = new_shape.to_vec();

        self.recalculate_strides();
        self.recalculate_backstrides();
        self.set_array_flags();
    }

    fn transpose(&self, axes:Option<&[usize]>) -> Array{
        let mut res = self.clone();

        if self.ndim == 1 {
            return res;
        }

        let axes = match axes{
            Some(a) => a.to_vec(),
            None => (0..self.ndim).rev().collect(),
        };

        res.shape = axes.iter().map(|&i| self.shape[i]).collect();  // closures are used here
        res.strides = axes.iter().map(|&i| self.strides[i]).collect();

        res.recalculate_backstrides();
        res.set_array_flags();

        return res;
    }

    fn add(&self, other: &Array) -> Array {
        if self.check_shape_equal(other) {
            return self.elementwise_op(other, |a, b| a + b);
        }

        let res_shape = self.final_shape(other).expect("Can't add arrays of non broadcastable shapes");
        let a_final = self.broadcast(&res_shape);
        let b_final = other.broadcast(&res_shape);

        return a_final.elementwise_op(&b_final, |a, b| a + b);
    }

    fn mul(&self, other:Array) -> Array {
        if self.check_shape_equal(&other) {
            return self.elementwise_op(&other, |a, b| a * b);
        }

        let res_shape = self.final_shape(&other).expect("Can't multiply arrays of non broadcastable shapes");
        let a_final = self.broadcast(&res_shape);
        let b_final = other.broadcast(&res_shape);

        return a_final.elementwise_op(&b_final, |a, b| a * b);
    }

    fn elementwise_op<T>(&self, other: &Array, op: T) -> Array
    where T: Fn(f32, f32) -> f32,
    {
        assert_eq!(self.shape, other.shape);
        let mut res = self.clone();

        for i in 0..self.totalsize{
            res.data[i] = op(self.data[i], other.data[i]);
        }

        return res;
    }

    fn apply_inplace<T>(&mut self, func: T)
    where T: Fn(f32) -> f32,
    {
        for i in 0..self.totalsize{
            self.data[i] = func(self.data[i]);
        }
    }
}

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Array {:?} {:?}", self.shape, self.data)
    }
}

fn square(x: f32) -> f32{
    x*x
}

fn cube(x: f32) -> f32{
    x*x*x
}

fn main(){

    let shape = [3, 3];
    let mut arr1 = Array::random(&shape);
    let mut arr2 = Array::random(&shape);

    let mut arr3 = arr1.clone();
    println!("{:?}", arr1.add(&arr2));
    println!("{:?}",arr1);
    println!("{:?}",arr3);

    println!();
    println!("{:?}", arr1.mul(arr2));

    println!("{:?}", arr1.reshape(&[4, 4]));
    println!("{:?}", arr1.shape);
    //arr1.apply_inplace(square);
    //arr1.show();
}
