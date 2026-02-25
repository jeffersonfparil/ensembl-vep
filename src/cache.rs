use pyo3::prelude::*;

#[allow(dead_code)] 
#[pyclass]
pub struct OfflineCache {
    gff_path: String,
}

#[pymethods]
impl OfflineCache {
    #[new]
    pub fn new(gff_path: String) -> Self {
        OfflineCache { gff_path }
    }

    // TODO
    pub fn fetch_transcripts(&self, _chrom: &str, _start: u32) -> PyResult<Vec<String>> {
        // In a full implementation, this uses noodles::tabix to jump to the byte offset.
        // Mocking the return for the scaffold:
        Ok(vec!["ENST00000123456.1".to_string(), "ENST00000654321.1".to_string()])
    }
}