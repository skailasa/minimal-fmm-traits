use mpi::traits::Communicator;
use num_traits::Float;

use crate::{
    fmm::KiFmm,
    kernel,
    traits::{Kernel, ScaleInvariantHomogenousKernel, SourceToTargetData},
    tree::{MultiNodeFmmTree, MultiNodeTree, SingleNodeFmmTree, SingleNodeTree},
};

#[derive(Default)]
pub struct KiFmmBuilderSingleNode<'builder, T, U, V>
where
    T: SourceToTargetData,
    U: Float,
    V: Kernel,
{
    tree: Option<SingleNodeFmmTree<'builder, U>>,
    source_to_target: Option<T>,
    kernel: Option<V>,
    order: Option<usize>,
    max_depth: Option<usize>,
}

#[derive(Default)]
pub struct KiFmmBuilderMultiNode<'builder, T, U, V, W>
where
    T: SourceToTargetData,
    U: Communicator,
    V: Float,
    W: Kernel + ScaleInvariantHomogenousKernel,
{
    tree: Option<MultiNodeFmmTree<'builder, V>>,
    source_to_target: Option<T>,
    order: Option<usize>,
    comm: Option<U>,
    max_depth: Option<usize>,
    kernel: Option<W>,
}

impl<'builder, T, U, V> KiFmmBuilderSingleNode<'builder, T, U, V>
where
    T: SourceToTargetData,
    U: Float,
    V: Kernel,
{
    // Start building with mandatory parameters
    pub fn new() -> Self {
        KiFmmBuilderSingleNode {
            tree: None,
            source_to_target: None,
            kernel: None,
            order: None,
            max_depth: None,
        }
    }

    pub fn tree(
        mut self,
        sources: &'builder [U],
        targets: &'builder [U],
        n_crit: Option<usize>,
    ) -> Self {
        if n_crit.is_some() {
            let calculated_depth = 5;
            let source_tree = SingleNodeTree {
                points: sources,
                depth: calculated_depth,
            };
            let target_tree = SingleNodeTree {
                points: targets,
                depth: calculated_depth,
            };
            let fmm_tree = SingleNodeFmmTree {
                source_tree,
                target_tree,
            };
            self.tree = Some(fmm_tree);
            self
        } else {
            // Determine n crit from data
            let calculated_depth = 5;
            let source_tree = SingleNodeTree {
                points: sources,
                depth: calculated_depth,
            };
            let target_tree = SingleNodeTree {
                points: targets,
                depth: calculated_depth,
            };
            let max_depth = source_tree.depth.max(target_tree.depth);
            let fmm_tree = SingleNodeFmmTree {
                source_tree,
                target_tree,
            };
            self.tree = Some(fmm_tree);
            self.max_depth = Some(max_depth);
            self
        }
    }

    pub fn parameters(
        mut self,
        expansion_order: usize,
        mut source_to_target: T,
        kernel: V,
    ) -> Result<Self, String> {
        if self.tree.is_none() {
            Err("Must build tree before specifying FMM parameters".to_string())
        } else {
            source_to_target.set_expansion_order(expansion_order);
            source_to_target.calculate_m2l_operators(expansion_order, self.max_depth.unwrap());
            self.order = Some(expansion_order);
            self.source_to_target = Some(source_to_target);
            self.kernel = Some(kernel);
            Ok(self)
        }
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

impl<'builder, T, U, V, W> KiFmmBuilderMultiNode<'builder, T, U, V, W>
where
    T: SourceToTargetData,
    U: Communicator,
    V: Float,
    W: Kernel + ScaleInvariantHomogenousKernel,
{
    // Start building with mandatory parameters
    pub fn new() -> Self {
        KiFmmBuilderMultiNode {
            tree: None,
            source_to_target: None,
            comm: None,
            order: None,
            max_depth: None,
            kernel: None,
        }
    }

    pub fn tree(
        mut self,
        sources: &'builder [V],
        targets: &'builder [V],
        n_crit: Option<usize>,
        comm: U,
    ) -> Self {
        if n_crit.is_some() {
            let depth = 4;
            let source_tree = MultiNodeTree {
                points: sources,
                depth,
            };
            let target_tree = MultiNodeTree {
                points: targets,
                depth,
            };
            let max_depth = source_tree.depth.max(target_tree.depth);
            let comm = comm.duplicate();
            let fmm_tree = MultiNodeFmmTree {
                comm,
                source_tree,
                target_tree,
            };
            self.tree = Some(fmm_tree);
            self.max_depth = Some(max_depth);
            self
        } else {
            // Determine n crit from data
            let depth = 4;
            let source_tree = MultiNodeTree {
                points: sources,
                depth,
            };
            let target_tree = MultiNodeTree {
                points: targets,
                depth,
            };

            let max_depth = source_tree.depth.max(target_tree.depth);
            let comm = comm.duplicate();
            let fmm_tree = MultiNodeFmmTree {
                comm,
                source_tree,
                target_tree,
            };
            self.tree = Some(fmm_tree);
            self.max_depth = Some(max_depth);
            self
        }
    }

    pub fn parameters(
        mut self,
        expansion_order: usize,
        mut source_to_target: T,
        kernel: W,
    ) -> Result<Self, String> {
        if self.tree.is_none() {
            Err("Must build tree before specifying FMM parameters".to_string())
        } else {
            source_to_target.set_expansion_order(expansion_order);
            // This should be done in build step, so this can be passed around cheaply
            source_to_target.calculate_m2l_operators(expansion_order, self.max_depth.unwrap());
            self.source_to_target = Some(source_to_target);
            self.order = Some(expansion_order);
            self.kernel = Some(kernel);
            Ok(self)
        }
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
