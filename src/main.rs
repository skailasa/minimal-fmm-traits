use minimal_fmm_traits::{
    builder::{KiFmmBuilderMultiNode, KiFmmBuilderSingleNode},
    field_translations::{SourceToTargetDataFft, SourceToTargetDataSvd},
    kernel::LaplaceKernel,
    other::EvalType,
    traits::Fmm,
};

fn main() {
    let targets = [0f64];
    let sources = [0f64];
    let mut result = [0f64];
    let charges = [0f64];
    let expansion_order = 10;
    let n_crit = None;

    // Single node fmm
    {
        let fmm = KiFmmBuilderSingleNode::new()
            .tree(&sources, &targets, n_crit)
            .parameters(
                expansion_order,
                SourceToTargetDataSvd::new(),
                LaplaceKernel::new(),
            )
            .unwrap()
            .build()
            .unwrap();

        fmm.evaluate_vec(EvalType::Value, &charges, &mut result);
    }

    // Multi node fmm
    {
        let universe = mpi::initialize().unwrap();
        let world = universe.world();

        let fmm = KiFmmBuilderMultiNode::new()
            .tree(&sources, &targets, n_crit, world)
            .parameters(
                expansion_order,
                SourceToTargetDataFft::new(),
                LaplaceKernel::new(),
            )
            .unwrap()
            .build()
            .unwrap();

        fmm.evaluate_vec(EvalType::Value, &charges, &mut result);
        fmm.evaluate_mat(EvalType::Value, &charges, &mut result);
    }
}
