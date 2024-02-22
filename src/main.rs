use minimal_fmm_traits::{
    builder::{KiFmmBuilderMultiNode, KiFmmBuilderSingleNode},
    field_translations::{SourceToTargetDataFft, SourceToTargetDataSvd},
    other::EvalType,
    traits::Fmm,
};

fn main() {
    let targets = [0f64];
    let sources = [0f64];
    let charges = [0f64];
    let order = 10;
    let n_crit = 150; // Should get away without specifying, just choose a default decent value

    // Single node fmm
    {
        let fmm = KiFmmBuilderSingleNode::new()
            .tree(&targets, &sources, n_crit)
            .parameters(order, SourceToTargetDataSvd::new())
            .build()
            .unwrap();

        fmm.evaluate_vec(EvalType::Value, &charges);
    }

    // Multi node fmm
    {
        let universe = mpi::initialize().unwrap();
        let world = universe.world();

        let fmm = KiFmmBuilderMultiNode::new()
            .tree(&targets, &sources, n_crit, world)
            .parameters(order, SourceToTargetDataFft::new())
            .build()
            .unwrap();

        fmm.evaluate_vec(EvalType::Value, &charges);
    }
}
