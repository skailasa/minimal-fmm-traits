use crate::{
    field_translations::{SourceToTargetDataFft, SourceToTargetDataSvd},
    other::EvalType,
    traits::{
        Fmm, SourceToTarget, SourceToTargetData, SourceToTargetHomogenousScaleInvariant,
        SourceTranslation, TargetTranslation, Tree,
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

impl<T: Float> SourceToTarget for KiFmm<SingleNodeFmmTree<'_, T>, SourceToTargetDataSvd<T>> {
    fn m2l(&self, level: usize) {}
    fn p2l(&self, level: usize) {}
}

impl<T: Float> SourceToTargetHomogenousScaleInvariant
    for KiFmm<SingleNodeFmmTree<'_, T>, SourceToTargetDataSvd<T>>
{
    fn scale(&self) {}
}
impl<T: Float> SourceToTarget for KiFmm<SingleNodeFmmTree<'_, T>, SourceToTargetDataFft> {
    fn m2l(&self, level: usize) {}
    fn p2l(&self, level: usize) {}
}

impl<T: Float> SourceToTargetHomogenousScaleInvariant
    for KiFmm<SingleNodeFmmTree<'_, T>, SourceToTargetDataFft>
{
    fn scale(&self) {}
}

impl<T: Float> SourceToTarget for KiFmm<MultiNodeFmmTree<'_, T>, SourceToTargetDataSvd<T>> {
    fn m2l(&self, level: usize) {}
    fn p2l(&self, level: usize) {}
}

impl<T: Float> SourceToTargetHomogenousScaleInvariant
    for KiFmm<MultiNodeFmmTree<'_, T>, SourceToTargetDataSvd<T>>
{
    fn scale(&self) {}
}
impl<T: Float> SourceToTarget for KiFmm<MultiNodeFmmTree<'_, T>, SourceToTargetDataFft> {
    fn m2l(&self, level: usize) {}
    fn p2l(&self, level: usize) {}
}

impl<T: Float> SourceToTargetHomogenousScaleInvariant
    for KiFmm<MultiNodeFmmTree<'_, T>, SourceToTargetDataFft>
{
    fn scale(&self) {}
}

impl<'fmm, T, U> Fmm for KiFmm<MultiNodeFmmTree<'fmm, T>, U>
where
    T: Float,
    U: SourceToTargetData,
    Self: SourceToTargetHomogenousScaleInvariant,
{
    type T = T;

    fn evaluate_vec(&self, eval_type: EvalType, charges: &[Self::T], result: &mut [Self::T]) {
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

    fn evaluate_mat(&self, eval_type: EvalType, charges_mat: &[Self::T], result: &mut [Self::T]) {
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

impl<'fmm, T, U> Fmm for KiFmm<SingleNodeFmmTree<'fmm, T>, U>
where
    T: Float,
    U: SourceToTargetData,
    Self: SourceToTargetHomogenousScaleInvariant,
{
    type T = T;

    fn evaluate_vec(&self, eval_type: EvalType, charges: &[Self::T], result: &mut [Self::T]) {

        match eval_type {
            EvalType::Value => {
                println!("evaluating potentials single node with vector of charges",)
            }
            EvalType::ValueDeriv => {
                println!("evaluating potentials and derivatives with vector of charges",)
            }
        }
    }

    fn evaluate_mat(&self, eval_type: EvalType, charges_mat: &[Self::T], result: &mut [Self::T]) {
        match eval_type {
            EvalType::Value => {
                println!("evaluating potentials single node with matrix of charges",)
            }
            EvalType::ValueDeriv => {
                println!("evaluating potentials and derivatives single node with matrix of charges",)
            }
        }
    }
}
