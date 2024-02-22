use crate::traits::SourceToTargetData;

pub struct SourceToTargetDataSvd {
    pub order: usize,
}
pub struct SourceToTargetDataFft {
    pub order: usize,
}

impl SourceToTargetData for SourceToTargetDataSvd {
    fn set_order(&mut self, order: usize) {
        self.order = order;
    }
}
impl SourceToTargetData for SourceToTargetDataFft {
    fn set_order(&mut self, order: usize) {
        self.order = order;
    }
}

impl SourceToTargetDataFft {
    pub fn new() -> Self {
        SourceToTargetDataFft { order: 1 }
    }
}

impl SourceToTargetDataSvd {
    pub fn new() -> Self {
        SourceToTargetDataSvd { order: 1 }
    }
}
