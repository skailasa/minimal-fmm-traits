use mpi::traits::Communicator;
use num_traits::Float;

use crate::{
    domain::Domain3D,
    fmm::{ncoeffs, KiFmm},
    other::EvalType,
    traits::{Kernel, ScaleInvariantHomogenousKernel, SourceToTargetData, Tree},
    tree::{MultiNodeFmmTree, MultiNodeTree, SingleNodeFmmTree, SingleNodeTree},
};

#[derive(Default)]
pub struct KiFmmBuilderSingleNode<'builder, T, U, V>
where
    T: SourceToTargetData<V>,
    U: Float,
    V: Kernel,
{
    tree: Option<SingleNodeFmmTree<'builder, U>>,
    source_to_target: Option<T>,
    source_domain: Option<Domain3D>,
    target_domain: Option<Domain3D>,
    kernel: Option<V>,
    expansion_order: Option<usize>,
    ncoeffs: Option<usize>,
    max_depth: Option<usize>,
    eval_type: Option<EvalType>,
}

#[derive(Default)]
pub struct KiFmmBuilderMultiNode<'builder, T, U, V, W>
where
    T: SourceToTargetData<W, Domain = Domain3D>,
    U: Communicator,
    V: Float,
    W: Kernel + ScaleInvariantHomogenousKernel,
{
    tree: Option<MultiNodeFmmTree<'builder, V>>,
    source_domain: Option<Domain3D>,
    target_domain: Option<Domain3D>,
    source_to_target: Option<T>,
    expansion_order: Option<usize>,
    ncoeffs: Option<usize>,
    comm: Option<U>,
    max_depth: Option<usize>,
    kernel: Option<W>,
    eval_type: Option<EvalType>,
}

impl<'builder, T, U, V> KiFmmBuilderSingleNode<'builder, T, U, V>
where
    T: SourceToTargetData<V, Domain = Domain3D>,
    U: Float,
    V: Kernel + Clone,
{
    // Start building with mandatory parameters
    pub fn new() -> Self {
        KiFmmBuilderSingleNode {
            tree: None,
            source_domain: None,
            target_domain: None,
            source_to_target: None,
            kernel: None,
            expansion_order: None,
            ncoeffs: None,
            max_depth: None,
            eval_type: None,
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
                domain: Domain3D {},
            };
            let target_tree = SingleNodeTree {
                points: targets,
                depth: calculated_depth,
                domain: Domain3D {},
            };
            self.source_domain = Some(source_tree.get_domain());
            self.target_domain = Some(target_tree.get_domain());
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
                domain: Domain3D {},
            };
            let target_tree = SingleNodeTree {
                points: targets,
                depth: calculated_depth,
                domain: Domain3D {},
            };
            let max_depth = source_tree.depth.max(target_tree.depth);
            self.source_domain = Some(source_tree.get_domain());
            self.target_domain = Some(target_tree.get_domain());
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
        kernel: V,
        eval_type: EvalType,
    ) -> Result<Self, String> {
        if self.tree.is_none() {
            Err("Must build tree before specifying FMM parameters".to_string())
        } else {
            self.expansion_order = Some(expansion_order);
            self.ncoeffs = Some(ncoeffs(expansion_order));
            self.kernel = Some(kernel);
            self.eval_type = Some(eval_type);
            Ok(self)
        }
    }

    pub fn field_translation(mut self, mut source_to_target: T) -> Result<Self, String> {
        if self.expansion_order.is_none()
            || self.kernel.is_none()
            || self.ncoeffs.is_none()
            || self.source_domain.is_none()
        {
            Err("Must Build tree and specify FMM parameters".to_string())
        } else {
            // Set the expansion order
            source_to_target.set_expansion_order(self.expansion_order.unwrap());

            // Set the associated kernel
            let kernel = self.kernel.as_ref().unwrap().clone();
            source_to_target.set_kernel(kernel);

            // Compute the field translation operators
            source_to_target
                .set_operator_data(self.expansion_order.unwrap(), &self.source_domain.unwrap());

            self.source_to_target = Some(source_to_target);
            Ok(self)
        }
    }

    // Finalize and build the KiFmm
    pub fn build(self) -> Result<KiFmm<SingleNodeFmmTree<'builder, U>, T, V>, String> {
        if self.tree.is_none()
            || self.source_to_target.is_none()
            || self.expansion_order.is_none()
            || self.eval_type.is_none()
        {
            Err("Missing fields for KiFmm".to_string())
        } else {
            let eval_type = self.eval_type.unwrap();
            match eval_type {
                EvalType::Value => {
                    println!("Building Potentials FMM")
                }
                EvalType::ValueDeriv => {
                    println!("Building potentials + derivs FMM")
                }
            }

            Ok(KiFmm {
                tree: self.tree.unwrap(),
                field_translation_data: self.source_to_target.unwrap(),
                kernel: self.kernel.unwrap(),
                expansion_order: self.expansion_order.unwrap(),
                ncoeffs: self.ncoeffs.unwrap(),
            })
        }
    }
}

impl<'builder, T, U, V, W> KiFmmBuilderMultiNode<'builder, T, U, V, W>
where
    T: SourceToTargetData<W, Domain = Domain3D>,
    U: Communicator,
    V: Float,
    W: Kernel + ScaleInvariantHomogenousKernel + Clone,
{
    // Start building with mandatory parameters
    pub fn new() -> Self {
        KiFmmBuilderMultiNode {
            tree: None,
            source_domain: None,
            target_domain: None,
            source_to_target: None,
            comm: None,
            expansion_order: None,
            ncoeffs: None,
            max_depth: None,
            kernel: None,
            eval_type: None,
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
                domain: Domain3D {},
            };
            let target_tree = MultiNodeTree {
                points: targets,
                depth,
                domain: Domain3D {},
            };
            let max_depth = source_tree.depth.max(target_tree.depth);
            let comm = comm.duplicate();
            self.source_domain = Some(source_tree.get_domain());
            self.target_domain = Some(target_tree.get_domain());

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
                domain: Domain3D {},
            };
            let target_tree = MultiNodeTree {
                points: targets,
                depth,
                domain: Domain3D {},
            };

            let max_depth = source_tree.depth.max(target_tree.depth);
            let comm = comm.duplicate();
            self.source_domain = Some(source_tree.get_domain());
            self.target_domain = Some(target_tree.get_domain());

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
        kernel: W,
        eval_type: EvalType,
    ) -> Result<Self, String> {
        if self.tree.is_none() {
            Err("Must build tree before specifying FMM parameters".to_string())
        } else {
            self.expansion_order = Some(expansion_order);
            self.ncoeffs = Some(ncoeffs(expansion_order));
            self.kernel = Some(kernel);
            self.eval_type = Some(eval_type);
            Ok(self)
        }
    }

    pub fn field_translation(mut self, mut source_to_target: T) -> Result<Self, String> {
        if self.expansion_order.is_none() || self.kernel.is_none() || self.ncoeffs.is_none() {
            Err("Must Build tree and specify FMM parameters".to_string())
        } else {
            // Set the expansion order
            source_to_target.set_expansion_order(self.expansion_order.unwrap());

            // Set the associated kernel
            let kernel = self.kernel.as_ref().unwrap().clone();
            source_to_target.set_kernel(kernel);

            // Compute the field translation operators
            source_to_target
                .set_operator_data(self.expansion_order.unwrap(), &self.source_domain.unwrap());

            self.source_to_target = Some(source_to_target);
            Ok(self)
        }
    }

    // Finalize and build the KiFmm
    pub fn build(self) -> Result<KiFmm<MultiNodeFmmTree<'builder, V>, T, W>, String> {
        if self.tree.is_none()
            || self.source_to_target.is_none()
            || self.expansion_order.is_none()
            || self.eval_type.is_none()
        {
            Err("Missing fields for KiFmm".to_string())
        } else {
            let eval_type = self.eval_type.unwrap();
            match eval_type {
                EvalType::Value => {
                    println!("Building Potentials FMM")
                }
                EvalType::ValueDeriv => {
                    println!("Building potentials + derivs FMM")
                }
            }

            Ok(KiFmm {
                tree: self.tree.unwrap(),
                field_translation_data: self.source_to_target.unwrap(),
                kernel: self.kernel.unwrap(),
                expansion_order: self.expansion_order.unwrap(),
                ncoeffs: self.ncoeffs.unwrap(),
            })
        }
    }
}
