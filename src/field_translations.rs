use crate::traits::SourceToTargetData;

#[derive(Default)]
pub struct SourceToTargetDataSvd {
    pub expansion_order: usize,
}

#[derive(Default)]
pub struct SourceToTargetDataFft {
    pub expansion_order: usize,
}

impl SourceToTargetData for SourceToTargetDataSvd {
    fn set_expansion_order(&mut self, expansion_order: usize) {
        self.expansion_order = expansion_order
    }
}

impl SourceToTargetData for SourceToTargetDataFft {
    fn set_expansion_order(&mut self, expansion_order: usize) {
        self.expansion_order = expansion_order
    }
}

impl SourceToTargetDataFft {
    pub fn new() -> Self {
        SourceToTargetDataFft { expansion_order: 1 }
    }
}

impl SourceToTargetDataSvd {
    pub fn new() -> Self {
        SourceToTargetDataSvd { expansion_order: 1 }
    }
}
