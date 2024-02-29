

use crate::{
    domain::{Domain3D},
    operator_data::{FftOperatorData, SvdOperatorData},
    traits::{Kernel, SourceToTargetData},
};

#[derive(Default)]
pub struct SourceToTargetDataSvd<T>
where
    T: num_traits::Float,
{
    pub expansion_order: usize,
    pub threshold: T,
    pub operator_data: SvdOperatorData,
}

#[derive(Default)]
pub struct SourceToTargetDataFft {
    pub expansion_order: usize,
    pub operator_data: FftOperatorData,
}

impl<T, U> SourceToTargetData<U> for SourceToTargetDataSvd<T>
where
    T: num_traits::Float,
    U: Kernel,
{
    type OperatorData = SvdOperatorData;
    fn set_expansion_order(&mut self, expansion_order: usize) {
        self.expansion_order = expansion_order
    }

    fn set_operator_data(&mut self, _expansion_order: usize, _domain: &Domain3D) {
        self.operator_data = SvdOperatorData {}
    }
}

impl<T> SourceToTargetData<T> for SourceToTargetDataFft
where
    T: Kernel,
{
    type OperatorData = SvdOperatorData;

    fn set_expansion_order(&mut self, expansion_order: usize) {
        self.expansion_order = expansion_order
    }

    fn set_operator_data(&mut self, _expansion_order: usize, _domain: &Domain3D) {
        self.operator_data = FftOperatorData {}
    }
}

impl SourceToTargetDataFft {
    pub fn new() -> Self {
        SourceToTargetDataFft::default()
    }
}

impl<T> SourceToTargetDataSvd<T>
where
    T: num_traits::Float + Default,
{
    pub fn new(_threshold: T) -> Self {
        SourceToTargetDataSvd::<T>::default()
    }
}
