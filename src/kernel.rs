use crate::traits::{Kernel, ScaleInvariantHomogenousKernel};

pub struct LaplaceKernel {}

impl Kernel for LaplaceKernel {}

impl ScaleInvariantHomogenousKernel for LaplaceKernel {
    fn scale(&self) {}
}

impl LaplaceKernel {
    pub fn new() -> Self {
        LaplaceKernel {}
    }
}
