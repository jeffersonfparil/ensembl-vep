use pyo3::prelude::*;
use rust_lapper::{Interval, Lapper};

#[pyclass]
pub struct TranscriptTree {
    lapper: Lapper<u32, String>,
}

#[pymethods]
impl TranscriptTree {
    #[new]
    pub fn new() -> Self {
        // Mock data initialization
        let intervals = vec![
            Interval { start: 1000, stop: 2000, val: "GeneA".to_string() }
        ];
        TranscriptTree { lapper: Lapper::new(intervals) }
    }

    pub fn find_overlapping(&self, position: u32) -> PyResult<Vec<String>> {
        let overlaps = self.lapper.find(position, position + 1);
        Ok(overlaps.map(|ov| ov.val.clone()).collect())
    }
}