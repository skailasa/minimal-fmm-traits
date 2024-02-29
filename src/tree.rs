use mpi::topology::SimpleCommunicator;
use num_traits::Float;

use crate::traits::Tree;

pub struct MultiNodeTree<'tree, T: Float> {
    pub points: &'tree [T],
    pub depth: usize,
}

pub struct SingleNodeTree<'tree, T: Float> {
    pub points: &'tree [T],
    pub depth: usize,
}

pub struct SingleNodeFmmTree<'fmm_tree, T: Float> {
    pub source_tree: SingleNodeTree<'fmm_tree, T>,
    pub target_tree: SingleNodeTree<'fmm_tree, T>,
}

pub struct MultiNodeFmmTree<'fmm_tree, T: Float> {
    pub comm: SimpleCommunicator,
    pub source_tree: MultiNodeTree<'fmm_tree, T>,
    pub target_tree: MultiNodeTree<'fmm_tree, T>,
}

impl<U: Float> Tree for SingleNodeTree<'_, U> {}
impl<U: Float> Tree for MultiNodeTree<'_, U> {}
impl<U: Float> Tree for SingleNodeFmmTree<'_, U> {}
impl<U: Float> Tree for MultiNodeFmmTree<'_, U> {}
