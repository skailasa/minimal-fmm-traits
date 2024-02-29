use crate::{
    domain::Domain3D,
    operator_data::{FftOperatorData, SvdOperatorData},
    traits::{ScaleInvariantHomogenousKernel, SourceToTargetData},
};

pub struct TransferVector {}

#[derive(Default)]
pub struct SourceToTargetDataSvd<T, U>
where
    T: num_traits::Float,
    U: ScaleInvariantHomogenousKernel + Default,
{
    pub operator_data: SvdOperatorData,
    pub transfer_vectors: Vec<TransferVector>,
    expansion_order: usize,
    kernel: U,
    threshold: T,
}

#[derive(Default)]
pub struct SourceToTargetDataFft<T>
where
    T: ScaleInvariantHomogenousKernel + Default,
{
    pub operator_data: FftOperatorData,
    pub transfer_vectors: Vec<TransferVector>,
    pub surf_to_conv_map: Vec<usize>,
    pub conv_to_surf_map: Vec<usize>,
    expansion_order: usize, // expansion order is also private as it's only used in the computation of field translation data
    kernel: T, // kernel is private, as it's only used in the computation of field translation data
}

impl<T, U> SourceToTargetData<U> for SourceToTargetDataSvd<T, U>
where
    T: num_traits::Float,
    U: ScaleInvariantHomogenousKernel + Default,
{
    type OperatorData = SvdOperatorData;
    type Domain = Domain3D;

    fn set_expansion_order(&mut self, expansion_order: usize) {
        self.expansion_order = expansion_order
    }

    fn set_kernel(&mut self, kernel: U) {
        self.kernel = kernel
    }

    fn set_operator_data(&mut self, _expansion_order: usize, _domain: &Self::Domain) {
        self.operator_data = SvdOperatorData {}
    }
}

impl<T> SourceToTargetData<T> for SourceToTargetDataFft<T>
where
    T: ScaleInvariantHomogenousKernel + Default,
{
    type OperatorData = SvdOperatorData;
    type Domain = Domain3D;

    fn set_expansion_order(&mut self, expansion_order: usize) {
        self.expansion_order = expansion_order
    }

    fn set_kernel(&mut self, kernel: T) {
        self.kernel = kernel
    }

    fn set_operator_data(&mut self, _expansion_order: usize, _domain: &Self::Domain) {
        self.operator_data = FftOperatorData {}
    }
}

impl<T> SourceToTargetDataFft<T>
where
    T: ScaleInvariantHomogenousKernel + Default,
{
    pub fn new() -> Self {
        SourceToTargetDataFft::default()
    }
}

impl<T, U> SourceToTargetDataSvd<T, U>
where
    T: num_traits::Float + Default,
    U: ScaleInvariantHomogenousKernel + Default,
{
    pub fn new(_threshold: T) -> Self {
        SourceToTargetDataSvd::<T, U>::default()
    }
}
