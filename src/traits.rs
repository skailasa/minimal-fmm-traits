use crate::other::EvalType;

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

pub trait SourceToTargetHomogenous {
    fn m2l(&self, level: usize);
    fn p2l(&self, level: usize);
    fn scale(&self);
}

// Implemented over Concrete FMM
pub trait Fmm<T>
where
    Self: SourceTranslation + TargetTranslation + SourceToTargetHomogenous,
    T: num_traits::Float,
{
    fn evaluate_vec(&self, eval_type: EvalType, charges_vec: &[T], result: &mut [T]);

    fn evaluate_mat(&self, eval_type: EvalType, charges_mat: &[T], result: &mut [T]);
}

// Implemented over concrete tree
pub trait Tree {}

// Implemented over concrete kernel
pub trait ScaleInvariantKernel
where
    Self: Kernel,
{
    fn scale(&self);
}

pub trait Kernel {}

pub trait SourceToTargetData {
    fn set_expansion_order(&mut self, expansion_order: usize);
    fn calculate_m2l_operators(&mut self, expansion_order: usize, depth: usize);
}
