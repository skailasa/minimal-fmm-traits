use crate::types::EvalType;

// Implemented over DataTree
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
pub trait Fmm
where
    Self: SourceTranslation + TargetTranslation + SourceToTargetHomogenous
{
    fn evaluate(&self, eval_type: EvalType);
}

// Implemented over concrete tree
pub trait Tree {
}

// Implemented over concrete kernel
pub trait ScaleInvariantKernel
where
    Self: Kernel
{

    fn scale(&self);
}

pub trait Kernel {}

pub trait SourceToTargetData {}