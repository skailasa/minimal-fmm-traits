use crate::{domain::Domain3D, other::EvalType};

/// Implemented over type that contains metadata to perform FMM as well as a reference to the
/// source and target trees
pub trait SourceTranslation {
    fn p2m(&self);
    fn m2m(&self, level: usize);
}

/// Implemented over type that contains metadata to perform FMM as well as a reference to the
/// source and target trees
pub trait TargetTranslation {
    fn l2l(&self, level: usize);
    fn m2p(&self, level: usize);
    fn l2p(&self, level: usize);
    fn p2p(&self, level: usize);
}

/// Implemented over type that contains metadata to perform FMM as well as a reference to the
/// source and target trees
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

/// Implemented over Concrete FMM
/// T is an associated type, as we're not re-implementing this for different floating point types.
pub trait Fmm {
    type T: num_traits::Float;
    fn evaluate_vec(&self, eval_type: EvalType, charges_vec: &[Self::T], result: &mut [Self::T]);
    fn evaluate_mat(&self, eval_type: EvalType, charges_mat: &[Self::T], result: &mut [Self::T]);
    fn get_expansion_order(&self) -> usize;
    fn get_ncoeffs(&self) -> usize;
}

/// Implemented over concrete tree
/// Domain can be an associated type, as we're only implementing this on a specific type of tree
/// associted with a given domain
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

pub trait Kernel {
    fn evaluate_st(&self);

    fn evaluate_mt(&self);
}

/// template for each kernel as will need to re-implement the data structure
/// for storing operator data (e.g. helmholtz will have complex float types)
/// operator data itself will only be associated with a given kernel so can be stored
/// as an associated type
pub trait SourceToTargetData<T>
where
    T: Kernel,
{
    type OperatorData;

    fn set_expansion_order(&mut self, expansion_order: usize);
    fn set_operator_data(&mut self, expansion_order: usize, domain: &Domain3D);
    fn set_kernel(&mut self, kernel: T);
}
