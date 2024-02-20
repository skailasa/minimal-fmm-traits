use minimal_fmm_traits::{traits::Fmm, types::{EvalType, KiFmmBuilderMultiNode, KiFmmBuilderSingleNode, SourceToTargetDataFft, SourceToTargetDataSvd}};


fn main () {

    let targets = [0.];
    let sources = [0.];
    let charges = [0.];

    let fmm = KiFmmBuilderSingleNode::new()
        .tree(&targets, &sources, &charges)
        .translation_type(SourceToTargetDataSvd::new())
        .build()
        .unwrap();

    fmm.evaluate(EvalType::ValueDeriv);

    let fmm = KiFmmBuilderMultiNode::new()
        .tree(&targets, &sources, &charges)
        .translation_type(SourceToTargetDataFft::new())
        .build()
        .unwrap();

    fmm.evaluate(EvalType::Value);
}
