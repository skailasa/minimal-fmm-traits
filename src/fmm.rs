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

impl<U: Float> SourceTranslation for KiFmm<SingleNodeFmmTree<'_, U>, SourceToTargetDataSvd> {
    fn m2m(&self, level: usize) {}
    fn p2m(&self) {}
}

impl<U: Float> SourceTranslation for KiFmm<SingleNodeFmmTree<'_, U>, SourceToTargetDataFft> {
    fn m2m(&self, level: usize) {}
    fn p2m(&self) {}
}

impl<T: Float> SourceTranslation for KiFmm<MultiNodeFmmTree<'_, T>, SourceToTargetDataSvd> {
    fn m2m(&self, level: usize) {}
    fn p2m(&self) {}
}

impl<T: Float> SourceTranslation for KiFmm<MultiNodeFmmTree<'_, T>, SourceToTargetDataFft> {
    fn m2m(&self, level: usize) {}
    fn p2m(&self) {}
}

impl<T: Float> TargetTranslation for KiFmm<SingleNodeFmmTree<'_, T>, SourceToTargetDataSvd> {
    fn l2l(&self, level: usize) {}
    fn m2p(&self, level: usize) {}
    fn l2p(&self, level: usize) {}
    fn p2p(&self, level: usize) {}
}

impl<T: Float> TargetTranslation for KiFmm<SingleNodeFmmTree<'_, T>, SourceToTargetDataFft> {
    fn l2l(&self, level: usize) {}
    fn m2p(&self, level: usize) {}
    fn l2p(&self, level: usize) {}
    fn p2p(&self, level: usize) {}
}

impl<T: Float> TargetTranslation for KiFmm<MultiNodeFmmTree<'_, T>, SourceToTargetDataSvd> {
    fn l2l(&self, level: usize) {}
    fn m2p(&self, level: usize) {}
    fn l2p(&self, level: usize) {}
    fn p2p(&self, level: usize) {}
}

impl<T: Float> TargetTranslation for KiFmm<MultiNodeFmmTree<'_, T>, SourceToTargetDataFft> {
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

impl<'points, T> Fmm<T> for KiFmm<SingleNodeFmmTree<'points, T>, SourceToTargetDataFft>
where
    T: Float,
{
    fn evaluate_vec(&self, eval_type: EvalType, charges: &[T]) {
        match eval_type {
            EvalType::Value => println!("evaluating potentials"),
            EvalType::ValueDeriv => println!("evaluating potentials and derivatives"),
        }
    }

    fn evaluate_mat(&self, eval_type: EvalType, charges_mat: &[T]) {}
}

impl<'fmm, T> Fmm<T> for KiFmm<SingleNodeFmmTree<'fmm, T>, SourceToTargetDataSvd>
where
    T: Float,
{
    fn evaluate_vec(&self, eval_type: EvalType, charges: &[T]) {
        match eval_type {
            EvalType::Value => println!("evaluating potentials"),
            EvalType::ValueDeriv => println!("evaluating potentials and derivatives"),
        }
    }
    fn evaluate_mat(&self, eval_type: EvalType, charges_mat: &[T]) {}
}

impl<'fmm, T> Fmm<T> for KiFmm<MultiNodeFmmTree<'fmm, T>, SourceToTargetDataFft>
where
    T: Float,
{
    fn evaluate_vec(&self, eval_type: EvalType, charges: &[T]) {
        match eval_type {
            EvalType::Value => println!(
                "evaluating potentials multinode rank; {:?} order {:?}",
                self.tree.comm.rank(),
                self.m2l.order
            ),
            EvalType::ValueDeriv => println!(
                "evaluating potentials and derivatives multinode rank: {:?} order {:?}",
                self.tree.comm.rank(),
                self.m2l.order
            ),
        }
    }

    fn evaluate_mat(&self, eval_type: EvalType, charges_mat: &[T]) {}
}

impl<'fmm, T> Fmm<T> for KiFmm<MultiNodeFmmTree<'fmm, T>, SourceToTargetDataSvd>
where
    T: Float,
{
    fn evaluate_vec(&self, eval_type: EvalType, charges: &[T]) {
        match eval_type {
            EvalType::Value => println!("evaluating potentials multinode"),
            EvalType::ValueDeriv => println!("evaluating potentials and derivatives multinode"),
        }
    }

    fn evaluate_mat(&self, eval_type: EvalType, charges_mat: &[T]) {}
}
