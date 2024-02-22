use crate::traits::{Kernel, ScaleInvariantKernel};

pub struct LaplaceKernel {}

impl Kernel for LaplaceKernel {}

impl ScaleInvariantKernel for LaplaceKernel {
    fn scale(&self) {}
}
