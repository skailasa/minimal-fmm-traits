use mpi::topology::SimpleCommunicator;
use num_traits::Float;

use crate::traits::Tree;

pub struct MultiNodeTree<'tree, U: Float> {
    pub points: &'tree [U],
}

pub struct SingleNodeTree<'tree, U: Float> {
    pub points: &'tree [U],
}

pub struct SingleNodeFmmTree<'fmm_tree, U: Float> {
    pub source_tree: SingleNodeTree<'fmm_tree, U>,
    pub target_tree: SingleNodeTree<'fmm_tree, U>,
}

pub struct MultiNodeFmmTree<'fmm_tree, U: Float> {
    pub comm: SimpleCommunicator,
    pub source_tree: MultiNodeTree<'fmm_tree, U>,
    pub target_tree: MultiNodeTree<'fmm_tree, U>,
}

impl<U: Float> Tree for SingleNodeTree<'_, U> {}
impl<U: Float> Tree for MultiNodeTree<'_, U> {}
impl<U: Float> Tree for SingleNodeFmmTree<'_, U> {}
impl<U: Float> Tree for MultiNodeFmmTree<'_, U> {}
