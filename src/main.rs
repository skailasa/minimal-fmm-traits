use minimal_fmm_traits::{traits::Fmm, types::{EvalType, KiFmmBuilderSingleNode, SourceToTargetDataSvd, SourceToTargetDataFft}};


fn main () {

    let targets = [0.];
    let sources = [0.];
    let charges = [0.];

    let fmm = KiFmmBuilderSingleNode::new()
        .particle_data(&targets, &sources, &charges)
        .translation_type(SourceToTargetDataSvd::new())
        .build()
        .unwrap();

    fmm.run(EvalType::ValueDeriv);
}
