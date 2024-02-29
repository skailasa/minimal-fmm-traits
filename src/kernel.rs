use crate::traits::{Kernel, ScaleInvariantHomogenousKernel};

#[derive(Default, Clone)]
pub struct LaplaceKernel {}

impl Kernel for LaplaceKernel {
    fn evaluate_mt(&self) {}
    fn evaluate_st(&self) {}
}

impl ScaleInvariantHomogenousKernel for LaplaceKernel {
    fn scale(&self) {}
}

impl LaplaceKernel {
    pub fn new() -> Self {
        LaplaceKernel {}
    }
}
