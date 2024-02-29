use mpi::topology::SimpleCommunicator;
use num_traits::Float;

use crate::{
    domain::Domain3D,
    traits::{FmmTree, Tree},
};

pub struct MultiNodeTree<'tree, T: Float> {
    pub points: &'tree [T],
    pub depth: usize,
    pub domain: Domain3D,
}

pub struct SingleNodeTree<'tree, T: Float> {
    pub points: &'tree [T],
    pub depth: usize,
    pub domain: Domain3D,
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

impl<'fmm_tree, T: Float> FmmTree for SingleNodeFmmTree<'fmm_tree, T> {
    type Tree = SingleNodeTree<'fmm_tree, T>;

    fn get_source_tree(&self) -> &Self::Tree {
        &self.source_tree
    }

    fn get_target_tree(&self) -> &Self::Tree {
        &self.target_tree
    }
}

impl<'fmm_tree, T: Float> FmmTree for MultiNodeFmmTree<'fmm_tree, T> {
    type Tree = MultiNodeTree<'fmm_tree, T>;

    fn get_source_tree(&self) -> &Self::Tree {
        &self.source_tree
    }

    fn get_target_tree(&self) -> &Self::Tree {
        &self.target_tree
    }
}

impl<U: Float> Tree for SingleNodeTree<'_, U> {
    type Domain = Domain3D;
    fn get_domain(&self) -> Self::Domain {
        Domain3D {}
    }
}
impl<U: Float> Tree for MultiNodeTree<'_, U> {
    type Domain = Domain3D;
    fn get_domain(&self) -> Self::Domain {
        Domain3D {}
    }
}
impl<U: Float> Tree for SingleNodeFmmTree<'_, U> {
    type Domain = Domain3D;
    fn get_domain(&self) -> Self::Domain {
        Domain3D {}
    }
}
impl<U: Float> Tree for MultiNodeFmmTree<'_, U> {
    type Domain = Domain3D;
    fn get_domain(&self) -> Self::Domain {
        Domain3D {}
    }
}
