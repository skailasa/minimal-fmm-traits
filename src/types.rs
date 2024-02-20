use num_traits::Float;

use crate::traits::{Fmm, Kernel, ScaleInvariantKernel, SourceToTargetData, SourceToTargetHomogenous, SourceTranslation, TargetTranslation, Tree};

pub enum EvalType {
    Value,
    ValueDeriv,
}

pub enum  M2lType {
    Svd,
    Fft
}

pub enum TreeType {
    SingleNode,
    MultiNode,
}

// Contains tree + kernel + metadata required to compute FMM
pub struct KiFmm<T: Tree, U: SourceToTargetData> {
    pub tree: T,
    pub m2l: U
}

pub struct MultiNodeTree {}
pub struct SingleNodeTree {}

pub struct SingleNodeFmmTree {
    source_tree: SingleNodeTree,
    target_tree: SingleNodeTree
}

pub struct MultiNodeFmmTree {
    source_tree: MultiNodeTree,
    target_tree: MultiNodeTree
}

pub struct SourceToTargetDataSvd {}
pub struct SourceToTargetDataFft{}


pub struct LaplaceKernel {}

impl Kernel for LaplaceKernel {}

impl ScaleInvariantKernel for LaplaceKernel {
    fn scale(&self) {}
}

impl Tree for SingleNodeTree {}
impl Tree for MultiNodeTree {}
impl Tree for SingleNodeFmmTree {}
impl Tree for MultiNodeFmmTree {}

impl SourceTranslation for KiFmm<SingleNodeFmmTree, SourceToTargetDataSvd> {
    fn m2m(&self, level: usize) {}
    fn p2m(&self) {}
}

impl SourceTranslation for KiFmm<SingleNodeFmmTree, SourceToTargetDataFft> {
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

impl SourceToTargetData for SourceToTargetDataSvd {}
impl SourceToTargetData for SourceToTargetDataFft {}


impl TargetTranslation for KiFmm<SingleNodeFmmTree, SourceToTargetDataSvd> {
    fn l2l(&self, level: usize) {}
    fn m2p(&self, level: usize) {}
    fn l2p(&self, level: usize) {}
    fn p2p(&self, level: usize) {}
}

impl TargetTranslation for KiFmm<SingleNodeFmmTree, SourceToTargetDataFft> {
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

impl SourceToTargetHomogenous for KiFmm<SingleNodeFmmTree, SourceToTargetDataSvd> {
    fn m2l(&self, level: usize) {}
    fn p2l(&self, level: usize) {}
    fn scale(&self) {}
}

impl SourceToTargetHomogenous for KiFmm<SingleNodeFmmTree, SourceToTargetDataFft> {
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
        SourceToTargetDataFft {}
    }
}

impl SourceToTargetDataSvd {
    pub fn new() -> Self {
        SourceToTargetDataSvd {}
    }
}

pub struct KiFmmBuilderSingleNode<T>
where
    T: SourceToTargetData,
{
    tree: Option<SingleNodeFmmTree>,
    m2l: Option<T>,
}

pub struct KiFmmBuilderMultiNode<T>
where
    T: SourceToTargetData,
{
    tree: Option<MultiNodeFmmTree>,
    m2l: Option<T>,
}

impl<U> KiFmmBuilderSingleNode<U>
where
    U: SourceToTargetData,
{
    // Start building with mandatory parameters
    pub fn new() -> Self {
        KiFmmBuilderSingleNode {
            tree: None,
            m2l: None,
        }
    }

    pub fn tree<T: Float>(mut self, targets: &[T], sources: &[T], charges: &[T]) -> Self {

        let source_tree = SingleNodeTree {};
        let target_tree = SingleNodeTree {};
        let fmm_tree = SingleNodeFmmTree {
            source_tree,
            target_tree
        };
        self.tree = Some(fmm_tree);
        self
    }

    pub fn expansions(mut self, order: usize, m2l: U) -> Self {
        self.m2l = Some(m2l);
        self
    }

    // Finalize and build the KiFmm
    pub fn build(self) -> Result<KiFmm<SingleNodeFmmTree, U>, String> {
        if self.tree.is_none() || self.m2l.is_none() {
            Err("Missing fields for KiFmm".to_string())
        } else {
            Ok(KiFmm {
                tree: self.tree.unwrap(),
                m2l: self.m2l.unwrap(),
            })
        }
    }
}

impl<U> KiFmmBuilderMultiNode<U>
where
    U: SourceToTargetData,
{
    // Start building with mandatory parameters
    pub fn new() -> Self {
        KiFmmBuilderMultiNode {
            tree: None,
            m2l: None,
        }
    }

    pub fn tree<T: Float>(mut self, targets: &[T], sources: &[T], charges: &[T]) -> Self {

        let source_tree = MultiNodeTree {};
        let target_tree = MultiNodeTree {};
        let fmm_tree = MultiNodeFmmTree {
            source_tree,
            target_tree
        };
        self.tree = Some(fmm_tree);
        self
    }

    pub fn expansions(mut self, order: usize, m2l: U) -> Self {
        self.m2l = Some(m2l);
        self
    }

    // Finalize and build the KiFmm
    pub fn build(self) -> Result<KiFmm<MultiNodeFmmTree, U>, String> {
        if self.tree.is_none() || self.m2l.is_none() {
            Err("Missing fields for KiFmm".to_string())
        } else {
            Ok(KiFmm {
                tree: self.tree.unwrap(),
                m2l: self.m2l.unwrap(),
            })
        }
    }
}

impl Fmm for KiFmm<SingleNodeFmmTree, SourceToTargetDataFft> {
    fn evaluate(&self, eval_type: EvalType) {

        match eval_type {
            EvalType::Value  => println!("evaluating potentials"),
            EvalType::ValueDeriv => println!("evaluating potentials and derivatives")
        }
    }
}


impl Fmm for KiFmm<SingleNodeFmmTree, SourceToTargetDataSvd> {
    fn evaluate(&self, eval_type: EvalType) {

        match eval_type {
            EvalType::Value  => println!("evaluating potentials"),
            EvalType::ValueDeriv => println!("evaluating potentials and derivatives")
        }
    }
}

impl Fmm for KiFmm<MultiNodeFmmTree, SourceToTargetDataFft> {
    fn evaluate(&self, eval_type: EvalType) {

        match eval_type {
            EvalType::Value  => println!("evaluating potentials multinode"),
            EvalType::ValueDeriv => println!("evaluating potentials and derivatives multinode")
        }
    }
}


impl Fmm for KiFmm<MultiNodeFmmTree, SourceToTargetDataSvd> {
    fn evaluate(&self, eval_type: EvalType) {

        match eval_type {
            EvalType::Value  => println!("evaluating potentials multinode"),
            EvalType::ValueDeriv => println!("evaluating potentials and derivatives multinode")
        }
    }
}
