use core::num;

use crate::traits::SourceToTargetData;

#[derive(Default)]
pub struct SourceToTargetDataSvd<T>
where
    T: num_traits::Float
{
    pub expansion_order: usize,
    pub threshold: T
}

#[derive(Default)]
pub struct SourceToTargetDataFft {
    pub expansion_order: usize,
}

impl <T>SourceToTargetData for SourceToTargetDataSvd<T>
where
    T: num_traits::Float
{
    fn set_expansion_order(&mut self, expansion_order: usize) {
        self.expansion_order = expansion_order
    }

    fn set_metadata(&mut self, expansion_order: usize, depth: usize) {}
}

impl SourceToTargetData for SourceToTargetDataFft {
    fn set_expansion_order(&mut self, expansion_order: usize) {
        self.expansion_order = expansion_order
    }

    fn set_metadata(&mut self, expansion_order: usize, depth: usize) {}
}

impl SourceToTargetDataFft {
    pub fn new() -> Self {
        SourceToTargetDataFft { expansion_order: 1 }
    }
}

impl <T>SourceToTargetDataSvd<T>
where
    T: num_traits::Float
{
    pub fn new(threshold: T) -> Self {
        SourceToTargetDataSvd::<T> { expansion_order: 1, threshold}
    }
}
