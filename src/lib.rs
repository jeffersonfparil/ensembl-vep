#![allow(non_local_definitions)]

use pyo3::prelude::*;

mod cache;
mod consequences;
mod intervals;

use cache::OfflineCache;
use consequences::VariationFeature;
use intervals::TranscriptTree;

#[pymodule]
fn vep_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<OfflineCache>()?;
    m.add_class::<VariationFeature>()?;
    m.add_class::<TranscriptTree>()?;
    Ok(())
}