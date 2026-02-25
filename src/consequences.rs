use pyo3::prelude::*;

#[pyclass]
pub struct VariationFeature {
    #[pyo3(get)] pub chrom: String,
    #[pyo3(get)] pub pos: u32,
    #[pyo3(get)] pub ref_allele: String,
    #[pyo3(get)] pub alt_allele: String,
}

#[pymethods]
impl VariationFeature {
    #[new]
    pub fn new(chrom: String, pos: u32, ref_allele: String, alt_allele: String) -> Self {
        VariationFeature { chrom, pos, ref_allele, alt_allele }
    }

    pub fn calculate_consequence(&self) -> PyResult<String> {
        // Simplified core biological logic
        if self.ref_allele.len() == self.alt_allele.len() {
            Ok("missense_variant".to_string())
        } else if self.ref_allele.len() > self.alt_allele.len() {
            Ok("frameshift_variant".to_string())
        } else {
            Ok("inframe_insertion".to_string())
        }
    }
}