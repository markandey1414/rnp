//extern crate macro_attr;

#[macro_export]
macro_rules! rar{
    ($name:ident, $len:expr) =>{
        struct $name <T>{
            data: [T; $len],
        }

        impl <T> $name <T> 
        where T: Copy + Default 
            + std::ops::Add<Output = T> 
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::fmt::Debug,
            {
            pub fn new(data: [T; $len]) ->Self{
                Self{data}
            }

            pub fn add(&self, other:&$name<T>) -> $name<T>{
                let mut result:[T; $len] = [T::default(); $len];
                for idx in 0..$len{
                    result[idx] = self.data[idx] + other.data[idx];
                }
                $name::new(result)
            }
            pub fn sub(&self, other:&$name<T>) -> $name<T>{
                let mut result:[T; $len] = [T::default(); $len];
                for idx in 0..$len{
                    result[idx] = self.data[idx] - other.data[idx];
                }
                $name::new(result)
            }
            pub fn dot(&self, other:&$name<T>) -> T{
                let mut result = T::default();
                for idx in 0..$len{
                    result = result + (self.data[idx] * other.data[idx]);
                }
                result
            }
        }
    }
}