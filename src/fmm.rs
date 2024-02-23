use crate::{
    field_translations::{SourceToTargetDataFft, SourceToTargetDataSvd},
    other::EvalType,
    traits::{
        Fmm, SourceToTargetData, SourceToTargetHomogenous, SourceTranslation, TargetTranslation,
        Tree,
    },
    tree::{MultiNodeFmmTree, SingleNodeFmmTree},
};
use num_traits::Float;

use mpi::topology::Communicator;

// Contains tree + kernel + metadata required to compute FMM
pub struct KiFmm<T: Tree, U: SourceToTargetData> {
    pub tree: T,
    pub m2l: U,
}

impl<T: Float, U: SourceToTargetData> SourceTranslation for KiFmm<SingleNodeFmmTree<'_, T>, U> {
    fn m2m(&self, level: usize) {}
    fn p2m(&self) {}
}

impl<T: Float, U: SourceToTargetData> SourceTranslation for KiFmm<MultiNodeFmmTree<'_, T>, U> {
    fn m2m(&self, level: usize) {}
    fn p2m(&self) {}
}

impl<T: Float, U: SourceToTargetData> TargetTranslation for KiFmm<SingleNodeFmmTree<'_, T>, U> {
    fn l2l(&self, level: usize) {}
    fn m2p(&self, level: usize) {}
    fn l2p(&self, level: usize) {}
    fn p2p(&self, level: usize) {}
}

impl<T: Float, U: SourceToTargetData> TargetTranslation for KiFmm<MultiNodeFmmTree<'_, T>, U> {
    fn l2l(&self, level: usize) {}
    fn m2p(&self, level: usize) {}
    fn l2p(&self, level: usize) {}
    fn p2p(&self, level: usize) {}
}

impl<T: Float> SourceToTargetHomogenous for KiFmm<SingleNodeFmmTree<'_, T>, SourceToTargetDataSvd> {
    fn m2l(&self, level: usize) {}
    fn p2l(&self, level: usize) {}
    fn scale(&self) {}
}

impl<T: Float> SourceToTargetHomogenous for KiFmm<SingleNodeFmmTree<'_, T>, SourceToTargetDataFft> {
    fn m2l(&self, level: usize) {}
    fn p2l(&self, level: usize) {}
    fn scale(&self) {}
}

impl<T: Float> SourceToTargetHomogenous for KiFmm<MultiNodeFmmTree<'_, T>, SourceToTargetDataSvd> {
    fn m2l(&self, level: usize) {}
    fn p2l(&self, level: usize) {}
    fn scale(&self) {}
}

impl<T: Float> SourceToTargetHomogenous for KiFmm<MultiNodeFmmTree<'_, T>, SourceToTargetDataFft> {
    fn m2l(&self, level: usize) {}
    fn p2l(&self, level: usize) {}
    fn scale(&self) {}
}

impl<'fmm, T, U: SourceToTargetData> Fmm<T> for KiFmm<SingleNodeFmmTree<'fmm, T>, U>
where
    T: Float,
    Self: SourceToTargetHomogenous,
{
    fn evaluate_vec(&self, eval_type: EvalType, charges: &[T], result: &mut [T]) {
        match eval_type {
            EvalType::Value => println!("evaluating potentials"),
            EvalType::ValueDeriv => println!("evaluating potentials and derivatives"),
        }
    }
    fn evaluate_mat(&self, eval_type: EvalType, charges_mat: &[T], result: &mut [T]) {}
}

impl<'fmm, T, U> Fmm<T> for KiFmm<MultiNodeFmmTree<'fmm, T>, U>
where
    T: Float,
    U: SourceToTargetData,
    Self: SourceToTargetHomogenous,
{
    fn evaluate_vec(&self, eval_type: EvalType, charges: &[T], result: &mut [T]) {
        match eval_type {
            EvalType::Value => println!(
                "evaluating potentials multinode rank; {:?} with vector of charges",
                self.tree.comm.rank(),
            ),
            EvalType::ValueDeriv => println!(
                "evaluating potentials and derivatives multinode rank: {:?}  with vector of charges",
                self.tree.comm.rank(),
            ),
        }
    }

    fn evaluate_mat(&self, eval_type: EvalType, charges_mat: &[T], result: &mut [T]) {
        match eval_type {
            EvalType::Value => println!(
                "evaluating potentials multinode rank; {:?} with matrix of charges",
                self.tree.comm.rank(),
            ),
            EvalType::ValueDeriv => println!(
                "evaluating potentials and derivatives multinode rank: {:?} with matrix of charges",
                self.tree.comm.rank(),
            ),
        }
    }
}
