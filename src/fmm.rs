use crate::{
    field_translations::{SourceToTargetDataFft, SourceToTargetDataSvd},
    other::EvalType,
    traits::{
        Fmm, Kernel, ScaleInvariantHomogenousKernel, SourceToTarget, SourceToTargetData,
        SourceToTargetHomogenousScaleInvariant, SourceTranslation, TargetTranslation, Tree,
    },
    tree::{MultiNodeFmmTree, SingleNodeFmmTree},
};
use num_traits::Float;

use mpi::topology::Communicator;

// Contains tree + kernel + metadata required to compute FMM
pub struct KiFmm<T: Tree, U: SourceToTargetData<V>, V: Kernel> {
    pub tree: T,
    pub field_translation_data: U,
    pub kernel: V,
    pub expansion_order: usize,
    pub ncoeffs: usize,
}

pub fn ncoeffs(expansion_order: usize) -> usize {
    6 * (expansion_order - 1).pow(2) + 2
}

impl<T: Float, U: SourceToTargetData<V>, V: Kernel> SourceTranslation
    for KiFmm<SingleNodeFmmTree<'_, T>, U, V>
{
    fn m2m(&self, _level: usize) {}
    fn p2m(&self) {}
}

impl<T: Float, U: SourceToTargetData<V>, V: Kernel> SourceTranslation
    for KiFmm<MultiNodeFmmTree<'_, T>, U, V>
{
    fn m2m(&self, _level: usize) {}
    fn p2m(&self) {}
}

impl<T: Float, U: SourceToTargetData<V>, V: Kernel> TargetTranslation
    for KiFmm<SingleNodeFmmTree<'_, T>, U, V>
{
    fn l2l(&self, _level: usize) {}
    fn m2p(&self, _level: usize) {}
    fn l2p(&self, _level: usize) {}
    fn p2p(&self, _level: usize) {}
}

impl<T: Float, U: SourceToTargetData<V>, V: Kernel> TargetTranslation
    for KiFmm<MultiNodeFmmTree<'_, T>, U, V>
{
    fn l2l(&self, _level: usize) {}
    fn m2p(&self, _level: usize) {}
    fn l2p(&self, _level: usize) {}
    fn p2p(&self, _level: usize) {}
}

impl<T: Float, U: ScaleInvariantHomogenousKernel + Default> SourceToTarget
    for KiFmm<SingleNodeFmmTree<'_, T>, SourceToTargetDataSvd<T, U>, U>
{
    fn m2l(&self, _level: usize) {}
    fn p2l(&self, _level: usize) {}
}

impl<T: Float, U: ScaleInvariantHomogenousKernel + Default> SourceToTargetHomogenousScaleInvariant
    for KiFmm<SingleNodeFmmTree<'_, T>, SourceToTargetDataSvd<T, U>, U>
{
    fn scale(&self) {}
}
impl<T: Float, U: ScaleInvariantHomogenousKernel + Default> SourceToTarget
    for KiFmm<SingleNodeFmmTree<'_, T>, SourceToTargetDataFft<U>, U>
{
    fn m2l(&self, _level: usize) {}
    fn p2l(&self, _level: usize) {}
}

impl<T: Float, U: ScaleInvariantHomogenousKernel + Default> SourceToTargetHomogenousScaleInvariant
    for KiFmm<SingleNodeFmmTree<'_, T>, SourceToTargetDataFft<U>, U>
{
    fn scale(&self) {}
}

impl<T: Float, U: ScaleInvariantHomogenousKernel + Default> SourceToTarget
    for KiFmm<MultiNodeFmmTree<'_, T>, SourceToTargetDataSvd<T, U>, U>
{
    fn m2l(&self, _level: usize) {}
    fn p2l(&self, _level: usize) {}
}

impl<T: Float, U: ScaleInvariantHomogenousKernel + Default> SourceToTargetHomogenousScaleInvariant
    for KiFmm<MultiNodeFmmTree<'_, T>, SourceToTargetDataSvd<T, U>, U>
{
    fn scale(&self) {}
}

impl<T: Float, U: ScaleInvariantHomogenousKernel + Default> SourceToTarget
    for KiFmm<MultiNodeFmmTree<'_, T>, SourceToTargetDataFft<U>, U>
{
    fn m2l(&self, _level: usize) {}
    fn p2l(&self, _level: usize) {}
}

impl<T: Float, U: ScaleInvariantHomogenousKernel + Default> SourceToTargetHomogenousScaleInvariant
    for KiFmm<MultiNodeFmmTree<'_, T>, SourceToTargetDataFft<U>, U>
{
    fn scale(&self) {}
}

impl<'fmm, T, U, V> Fmm for KiFmm<MultiNodeFmmTree<'fmm, T>, U, V>
where
    T: Float,
    U: SourceToTargetData<V>,
    V: Kernel,
    Self: SourceToTargetHomogenousScaleInvariant,
{
    type T = T;

    fn evaluate_vec(&self, eval_type: EvalType, _charges: &[Self::T], _result: &mut [Self::T]) {
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

    fn evaluate_mat(&self, eval_type: EvalType, _charges_mat: &[Self::T], _result: &mut [Self::T]) {
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

    fn get_expansion_order(&self) -> usize {
        self.expansion_order
    }

    fn get_ncoeffs(&self) -> usize {
        self.ncoeffs
    }
}

impl<'fmm, T, U, V> Fmm for KiFmm<SingleNodeFmmTree<'fmm, T>, U, V>
where
    T: Float,
    U: SourceToTargetData<V>,
    V: Kernel,
    Self: SourceToTargetHomogenousScaleInvariant,
{
    type T = T;

    fn evaluate_vec(&self, eval_type: EvalType, _charges: &[Self::T], _result: &mut [Self::T]) {
        match eval_type {
            EvalType::Value => {
                println!("evaluating potentials single node with vector of charges",)
            }
            EvalType::ValueDeriv => {
                println!("evaluating potentials and derivatives with vector of charges",)
            }
        }
    }

    fn evaluate_mat(&self, eval_type: EvalType, _charges_mat: &[Self::T], _result: &mut [Self::T]) {
        match eval_type {
            EvalType::Value => {
                println!("evaluating potentials single node with matrix of charges",)
            }
            EvalType::ValueDeriv => {
                println!("evaluating potentials and derivatives single node with matrix of charges",)
            }
        }
    }

    fn get_expansion_order(&self) -> usize {
        self.expansion_order
    }

    fn get_ncoeffs(&self) -> usize {
        self.ncoeffs
    }
}
