use mpi::topology::traits::Communicator;
use mpi::topology::SimpleCommunicator;
use mpi::{environment::Universe, traits::*};
use num_traits::Float;

use crate::traits::{
    Fmm, Kernel, ScaleInvariantKernel, SourceToTargetData, SourceToTargetHomogenous,
    SourceTranslation, TargetTranslation, Tree,
};

pub enum EvalType {
    Value,
    ValueDeriv,
}

// Contains tree + kernel + metadata required to compute FMM
pub struct KiFmm<T: Tree, U: SourceToTargetData> {
    pub tree: T,
    pub m2l: U,
}

pub struct MultiNodeTree {}

pub struct SingleNodeTree<'tree, U: Float> {points: &'tree [U]}

pub struct SingleNodeFmmTree<'fmm_tree, U: Float> {
    source_tree: SingleNodeTree<'fmm_tree, U>,
    target_tree: SingleNodeTree<'fmm_tree, U>,
}

pub struct MultiNodeFmmTree {
    comm: SimpleCommunicator,
    source_tree: MultiNodeTree,
    target_tree: MultiNodeTree,
}

pub struct SourceToTargetDataSvd {
    order: usize,
}
pub struct SourceToTargetDataFft {
    order: usize,
}

pub struct LaplaceKernel {}

impl Kernel for LaplaceKernel {}

impl ScaleInvariantKernel for LaplaceKernel {
    fn scale(&self) {}
}

impl <U: Float>Tree for SingleNodeTree<'_, U> {}
impl Tree for MultiNodeTree {}
impl <U: Float>Tree for SingleNodeFmmTree<'_, U> {}
impl Tree for MultiNodeFmmTree {}

impl <U: Float>SourceTranslation for KiFmm<SingleNodeFmmTree<'_, U>, SourceToTargetDataSvd> {
    fn m2m(&self, level: usize) {}
    fn p2m(&self) {}
}

impl <U: Float>SourceTranslation for KiFmm<SingleNodeFmmTree<'_, U>, SourceToTargetDataFft> {
    fn m2m(&self, level: usize) {}
    fn p2m(&self) {}
}

impl SourceTranslation for KiFmm<MultiNodeFmmTree, SourceToTargetDataSvd> {
    fn m2m(&self, level: usize) {}
    fn p2m(&self) {}
}

impl SourceTranslation for KiFmm<MultiNodeFmmTree, SourceToTargetDataFft> {
    fn m2m(&self, level: usize) {}
    fn p2m(&self) {}
}

impl SourceToTargetData for SourceToTargetDataSvd {
    fn set_order(&mut self, order: usize) {
        self.order = order;
    }
}
impl SourceToTargetData for SourceToTargetDataFft {
    fn set_order(&mut self, order: usize) {
        self.order = order;
    }
}

impl <U: Float>TargetTranslation for KiFmm<SingleNodeFmmTree<'_, U>, SourceToTargetDataSvd> {
    fn l2l(&self, level: usize) {}
    fn m2p(&self, level: usize) {}
    fn l2p(&self, level: usize) {}
    fn p2p(&self, level: usize) {}
}

impl <U: Float>TargetTranslation for KiFmm<SingleNodeFmmTree<'_, U>, SourceToTargetDataFft> {
    fn l2l(&self, level: usize) {}
    fn m2p(&self, level: usize) {}
    fn l2p(&self, level: usize) {}
    fn p2p(&self, level: usize) {}
}

impl TargetTranslation for KiFmm<MultiNodeFmmTree, SourceToTargetDataSvd> {
    fn l2l(&self, level: usize) {}
    fn m2p(&self, level: usize) {}
    fn l2p(&self, level: usize) {}
    fn p2p(&self, level: usize) {}
}

impl TargetTranslation for KiFmm<MultiNodeFmmTree, SourceToTargetDataFft> {
    fn l2l(&self, level: usize) {}
    fn m2p(&self, level: usize) {}
    fn l2p(&self, level: usize) {}
    fn p2p(&self, level: usize) {}
}

impl <U: Float>SourceToTargetHomogenous for KiFmm<SingleNodeFmmTree<'_, U>, SourceToTargetDataSvd> {
    fn m2l(&self, level: usize) {}
    fn p2l(&self, level: usize) {}
    fn scale(&self) {}
}

impl <U: Float>SourceToTargetHomogenous for KiFmm<SingleNodeFmmTree<'_, U>, SourceToTargetDataFft> {
    fn m2l(&self, level: usize) {}
    fn p2l(&self, level: usize) {}
    fn scale(&self) {}
}

impl SourceToTargetHomogenous for KiFmm<MultiNodeFmmTree, SourceToTargetDataSvd> {
    fn m2l(&self, level: usize) {}
    fn p2l(&self, level: usize) {}
    fn scale(&self) {}
}

impl SourceToTargetHomogenous for KiFmm<MultiNodeFmmTree, SourceToTargetDataFft> {
    fn m2l(&self, level: usize) {}
    fn p2l(&self, level: usize) {}
    fn scale(&self) {}
}

impl SourceToTargetDataFft {
    pub fn new() -> Self {
        SourceToTargetDataFft { order: 1 }
    }
}

impl SourceToTargetDataSvd {
    pub fn new() -> Self {
        SourceToTargetDataSvd { order: 1 }
    }
}

pub struct KiFmmBuilderSingleNode<'tree, T, U>
where
    T: SourceToTargetData,
    U: Float
{
    tree: Option<SingleNodeFmmTree<'tree, U>>,
    source_to_target: Option<T>,
}

pub struct KiFmmBuilderMultiNode<T, U>
where
    T: SourceToTargetData,
    U: Communicator,
{
    tree: Option<MultiNodeFmmTree>,
    source_to_target: Option<T>,
    order: Option<usize>,
    comm: Option<U>,
}

impl<'tree, U, T> KiFmmBuilderSingleNode<'tree, U, T>
where
    U: SourceToTargetData,
    T: Float
{
    // Start building with mandatory parameters
    pub fn new() -> Self {
        KiFmmBuilderSingleNode {
            tree: None,
            source_to_target: None,
        }
    }

    pub fn tree(mut self, targets: &'tree [T], sources: &'tree [T], n_crit: usize) -> Self {
        let source_tree = SingleNodeTree {points: sources};
        let target_tree = SingleNodeTree {points: targets};
        let fmm_tree = SingleNodeFmmTree {
            source_tree,
            target_tree,
        };
        self.tree = Some(fmm_tree);
        self
    }

    pub fn parameters(mut self, order: usize, mut source_to_target: U) -> Self {
        source_to_target.set_order(order);
        self.source_to_target = Some(source_to_target);
        self
    }

    // Finalize and build the KiFmm
    pub fn build(self) -> Result<KiFmm<SingleNodeFmmTree<'tree, T>, U>, String> {
        if self.tree.is_none() || self.source_to_target.is_none() {
            Err("Missing fields for KiFmm".to_string())
        } else {
            Ok(KiFmm {
                tree: self.tree.unwrap(),
                m2l: self.source_to_target.unwrap(),
            })
        }
    }
}

impl<'a, U, V> KiFmmBuilderMultiNode<U, V>
where
    U: SourceToTargetData,
    V: Communicator,
{
    // Start building with mandatory parameters
    pub fn new() -> Self {
        KiFmmBuilderMultiNode {
            tree: None,
            source_to_target: None,
            comm: None,
            order: None,
        }
    }

    pub fn mpi(mut self, comm: V) -> Self {
        self.comm = Some(comm);
        self
    }

    pub fn tree<T: Float>(mut self, targets: &[T], sources: &[T], n_crit: usize, comm: V) -> Self {
        let source_tree = MultiNodeTree {};
        let target_tree = MultiNodeTree {};

        let comm = comm.duplicate();
        let fmm_tree = MultiNodeFmmTree {
            comm,
            source_tree,
            target_tree,
        };
        self.tree = Some(fmm_tree);
        self
    }

    pub fn parameters(mut self, order: usize, mut source_to_target: U) -> Self {
        source_to_target.set_order(order);
        self.order = Some(order);
        self.source_to_target = Some(source_to_target);
        self
    }

    // Finalize and build the KiFmm
    pub fn build(self) -> Result<KiFmm<MultiNodeFmmTree, U>, String> {
        if self.tree.is_none() || self.source_to_target.is_none() {
            Err("Missing fields for KiFmm".to_string())
        } else {
            Ok(KiFmm {
                tree: self.tree.unwrap(),
                m2l: self.source_to_target.unwrap(),
            })
        }
    }
}

impl<'points, U> Fmm<U> for KiFmm<SingleNodeFmmTree<'points, U>, SourceToTargetDataFft>
where
    U: Float,
{
    fn evaluate_vec(&self, eval_type: EvalType, charges: &[U]) {
        match eval_type {
            EvalType::Value => println!("evaluating potentials"),
            EvalType::ValueDeriv => println!("evaluating potentials and derivatives"),
        }
    }

    fn evaluate_mat(&self, eval_type: EvalType, charges_mat: &[U]) {

    }
}

impl<'points, U> Fmm<U> for KiFmm<SingleNodeFmmTree<'points, U>, SourceToTargetDataSvd>
where
    U: Float,
{
    fn evaluate_vec(&self, eval_type: EvalType, charges: &[U]) {
        match eval_type {
            EvalType::Value => println!("evaluating potentials"),
            EvalType::ValueDeriv => println!("evaluating potentials and derivatives"),
        }
    }
    fn evaluate_mat(&self, eval_type: EvalType, charges_mat: &[U]) {
    }
}

impl<U> Fmm<U> for KiFmm<MultiNodeFmmTree, SourceToTargetDataFft>
where
    U: Float,
{
    fn evaluate_vec(&self, eval_type: EvalType, charges: &[U]) {
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

    fn evaluate_mat(&self, eval_type: EvalType, charges_mat: &[U]) {

    }
}

impl<U> Fmm<U> for KiFmm<MultiNodeFmmTree, SourceToTargetDataSvd>
where
    U: Float,
{
    fn evaluate_vec(&self, eval_type: EvalType, charges: &[U]) {
        match eval_type {
            EvalType::Value => println!("evaluating potentials multinode"),
            EvalType::ValueDeriv => println!("evaluating potentials and derivatives multinode"),
        }
    }

    fn evaluate_mat(&self, eval_type: EvalType, charges_mat: &[U]) {

    }
}
