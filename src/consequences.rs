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

    // TODO: This is a very simplified consequence calculation for demonstration purposes.
    pub fn calculate_consequence(&self) -> PyResult<String> {
        let ref_len = self.ref_allele.len();
        let alt_len = self.alt_allele.len();
        if ref_len == alt_len {
            // For now, we assume all same-length variants are missense
            Ok("missense_variant".to_string())
        } else if ref_len > alt_len {
            // Deletion logic
            if (ref_len - alt_len) % 3 == 0 {
                Ok("inframe_deletion".to_string())
            } else {
                Ok("frameshift_variant".to_string())
            }
        } else {
            // Insertion logic
            if (alt_len - ref_len) % 3 == 0 {
                Ok("inframe_insertion".to_string())
            } else {
                Ok("frameshift_variant".to_string())
            }
        }
    }
}