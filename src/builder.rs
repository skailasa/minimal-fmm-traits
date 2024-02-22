use mpi::traits::Communicator;
use num_traits::Float;

use crate::{
    fmm::KiFmm,
    traits::SourceToTargetData,
    tree::{MultiNodeFmmTree, MultiNodeTree, SingleNodeFmmTree, SingleNodeTree},
};

#[derive(Default)]
pub struct KiFmmBuilderSingleNode<'tree, T, U>
where
    T: SourceToTargetData,
    U: Float,
{
    tree: Option<SingleNodeFmmTree<'tree, U>>,
    source_to_target: Option<T>,
    order: Option<usize>,
}

#[derive(Default)]
pub struct KiFmmBuilderMultiNode<'builder, T, U, V>
where
    T: SourceToTargetData,
    U: Communicator,
    V: Float,
{
    tree: Option<MultiNodeFmmTree<'builder, V>>,
    source_to_target: Option<T>,
    order: Option<usize>,
    comm: Option<U>,
}

impl<'builder, T, U> KiFmmBuilderSingleNode<'builder, T, U>
where
    T: SourceToTargetData,
    U: Float,
{
    // Start building with mandatory parameters
    pub fn new() -> Self {
        KiFmmBuilderSingleNode {
            tree: None,
            source_to_target: None,
            order: None,
        }
    }

    pub fn tree(
        mut self,
        targets: &'builder [U],
        sources: &'builder [U],
        n_crit: Option<usize>,
    ) -> Self {
        if n_crit.is_some() {
            let source_tree = SingleNodeTree { points: sources };
            let target_tree = SingleNodeTree { points: targets };
            let fmm_tree = SingleNodeFmmTree {
                source_tree,
                target_tree,
            };
            self.tree = Some(fmm_tree);
            self
        } else {
            // Determine n crit from data
            let source_tree = SingleNodeTree { points: sources };
            let target_tree = SingleNodeTree { points: targets };
            let fmm_tree = SingleNodeFmmTree {
                source_tree,
                target_tree,
            };
            self.tree = Some(fmm_tree);
            self
        }
    }

    pub fn parameters(mut self, expansion_order: usize, mut source_to_target: T) -> Self {
        source_to_target.set_expansion_order(expansion_order);
        self.order = Some(expansion_order);
        self.source_to_target = Some(source_to_target);
        self
    }

    // Finalize and build the KiFmm
    pub fn build(self) -> Result<KiFmm<SingleNodeFmmTree<'builder, U>, T>, String> {
        if self.tree.is_none() || self.source_to_target.is_none() || self.order.is_none() {
            Err("Missing fields for KiFmm".to_string())
        } else {
            Ok(KiFmm {
                tree: self.tree.unwrap(),
                m2l: self.source_to_target.unwrap(),
            })
        }
    }
}

impl<'builder, T, U, V> KiFmmBuilderMultiNode<'builder, T, U, V>
where
    T: SourceToTargetData,
    U: Communicator,
    V: Float,
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

    pub fn mpi(mut self, comm: U) -> Self {
        self.comm = Some(comm);
        self
    }

    pub fn tree(
        mut self,
        targets: &'builder [V],
        sources: &'builder [V],
        n_crit: Option<usize>,
        comm: U,
    ) -> Self {
        if n_crit.is_some() {
            let source_tree = MultiNodeTree { points: sources };
            let target_tree = MultiNodeTree { points: targets };

            let comm = comm.duplicate();
            let fmm_tree = MultiNodeFmmTree {
                comm,
                source_tree,
                target_tree,
            };
            self.tree = Some(fmm_tree);
            self
        } else {
            // Determine n crit from data
            let source_tree = MultiNodeTree { points: sources };
            let target_tree = MultiNodeTree { points: targets };

            let comm = comm.duplicate();
            let fmm_tree = MultiNodeFmmTree {
                comm,
                source_tree,
                target_tree,
            };
            self.tree = Some(fmm_tree);
            self
        }
    }

    pub fn parameters(mut self, expansion_order: usize, mut source_to_target: T) -> Self {
        source_to_target.set_expansion_order(expansion_order);
        self.order = Some(expansion_order);
        self.source_to_target = Some(source_to_target);
        self
    }

    // Finalize and build the KiFmm
    pub fn build(self) -> Result<KiFmm<MultiNodeFmmTree<'builder, V>, T>, String> {
        if self.tree.is_none() || self.source_to_target.is_none() || self.order.is_none() {
            Err("Missing fields for KiFmm".to_string())
        } else {
            Ok(KiFmm {
                tree: self.tree.unwrap(),
                m2l: self.source_to_target.unwrap(),
            })
        }
    }
}
