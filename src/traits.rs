use crate::{domain::Domain3D, other::EvalType};

// Implemented over
pub trait SourceTranslation {
    fn p2m(&self);
    fn m2m(&self, level: usize);
}

pub trait TargetTranslation {
    fn l2l(&self, level: usize);
    fn m2p(&self, level: usize);
    fn l2p(&self, level: usize);
    fn p2p(&self, level: usize);
}

pub trait SourceToTarget {
    fn m2l(&self, level: usize);
    fn p2l(&self, level: usize);
}

pub trait SourceToTargetHomogenousScaleInvariant
where
    Self: SourceToTarget,
{
    fn scale(&self);
}

// Implemented over Concrete FMM
pub trait Fmm {
    type T: num_traits::Float;
    fn evaluate_vec(&self, eval_type: EvalType, charges_vec: &[Self::T], result: &mut [Self::T]);
    fn evaluate_mat(&self, eval_type: EvalType, charges_mat: &[Self::T], result: &mut [Self::T]);
    fn get_expansion_order(&self) -> usize;
    fn get_ncoeffs(&self) -> usize;
}

// Implemented over concrete tree
pub trait Tree {
    type Domain;
    fn get_domain(&self) -> Self::Domain;
}

// Implemented over concrete kernel
pub trait ScaleInvariantHomogenousKernel
where
    Self: Kernel,
{
    fn scale(&self);
}

pub trait Kernel {}


/// template for each kernel as will need to re-implement the data structure
// for storing operator data (e.g. helmholtz will have complex float types)
pub trait SourceToTargetData<T>
where
    T: Kernel,
{
    type OperatorData;

    fn set_expansion_order(&mut self, expansion_order: usize);
    fn set_operator_data(&mut self, expansion_order: usize, domain: &Domain3D);
}
