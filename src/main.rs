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
    let threshold = 0.1;
    let n_crit = None;

    // Single node fmm
    {
        let fmm = KiFmmBuilderSingleNode::new()
            .tree(&sources, &targets, n_crit)
            .parameters(expansion_order, LaplaceKernel::new())
            .unwrap()
            .field_translation(SourceToTargetDataSvd::new(threshold))
            .unwrap()
            .build()
            .unwrap();

        // For the n_crit can select default value based on uniform distributions.
        // then specify an FMM with a single parameter (order + threshold (for BLAS))
        fmm.evaluate_vec(EvalType::Value, &charges, &mut result);
    }

    // Multi node fmm
    {
        let universe = mpi::initialize().unwrap();
        let world = universe.world();

        // TODO: expect row major for coordinate data
        let fmm = KiFmmBuilderMultiNode::new()
            .tree(&sources, &targets, n_crit, world)
            .parameters(expansion_order, LaplaceKernel::new())
            .unwrap()
            .field_translation(SourceToTargetDataFft::new())
            .unwrap()
            .build()
            .unwrap();

        fmm.evaluate_vec(EvalType::Value, &charges, &mut result);
        fmm.evaluate_mat(EvalType::Value, &charges, &mut result);
    }
}
