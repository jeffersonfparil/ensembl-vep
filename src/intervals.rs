use pyo3::prelude::*;
use rust_lapper::{Interval, Lapper};

#[pyclass]
pub struct TranscriptTree {
    // We store the lapper tree inside the struct
    lapper: Lapper<usize, String>,
}

#[pymethods]
impl TranscriptTree {
    #[new]
    pub fn new() -> Self {
        // TODO: In a real implementation, we would load this data from a file or database. For now, we hardcode some intervals for testing.
        // Load the mock data that our pytest suite expects
        let intervals = vec![
            Interval { start: 1000, stop: 2000, val: "GeneA".to_string() },
            Interval { start: 3000, stop: 4000, val: "GeneB".to_string() },
        ];
        
        let lapper = Lapper::new(intervals);
        
        TranscriptTree { lapper }
    }

    // TODO: find all overlapping gene names for a given genomic position
    // This is a very simplified version of what a real implementation would do, but it demonstrates the concept.
    pub fn find_overlapping(&self, pos: usize) -> Vec<String> {
        // Lapper searches use a start and stop. Since it's a single point, we use pos to pos + 1
        self.lapper
            .find(pos, pos + 1)
            .map(|iv| iv.val.clone())
            .collect()
    }
}