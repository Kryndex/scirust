#![doc="Provides some signal generators
"]

// std imports


// external imports
use num::traits::{Zero, One};

// local imports
use sralgebra::CommutativeRingPartial;
use srmatrix::api::{Matrix, MatrixF64};

pub struct Impulse<T:CommutativeRingPartial> {
    /// The location at which impulse will come
    location : usize,
    // Current iteration index
    index : usize,
    #[allow(dead_code)]
    dummy : T
}

impl <T:CommutativeRingPartial> Impulse<T> {
    pub fn new(location: usize) -> Impulse<T>{
        Impulse{location : location, index : 0, dummy : Zero::zero()}
    }

    pub fn one(&self) -> T {
        One::one()
    }
}

impl <T:CommutativeRingPartial> Iterator for Impulse<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let v : T = if self.index == self.location {
            self.one()
        }
        else{
            Zero::zero()
        };
        self.index += 1;
        Some(v)
    }

}

pub fn impulse_f64() -> Impulse<f64> {
    Impulse::new(0)
}

pub fn  impulse_vector<T:CommutativeRingPartial>(length: usize, 
    location: usize) -> Matrix<T> {
        let gen : Impulse<T> = Impulse::new(location);
        Matrix::from_iter_cw(length, 1, gen)
}

pub fn impulse_vector_f64(length: usize, 
    location: usize) -> MatrixF64 {
    impulse_vector(length, location)
}

#[cfg(test)]
mod test{
    use super::*;
    use srmatrix::api::*;

    #[test]
    fn test_impulse_f64(){
        let mut gen = impulse_f64();
        for i in 0..100{
            let v = gen.next().unwrap();
            println!("v[{}], {}", i, v);
            if i == 0 {
                assert_eq!(v,1.);
            }
            else{
                assert_eq!(v,0.);
            }
        }
    }

    #[test]
    fn test_impulse_vector(){
        let  v1 = impulse_vector_f64(10, 2);
        let v2 = vector_f64(&[0., 0., 1., 0., 0.,
            0., 0., 0., 0., 0.]);
        assert_eq!(v1, v2);
    }

}
