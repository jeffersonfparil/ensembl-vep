# tests/test_rust_core.py
import pytest
from vep_pyrs.vep_core import VariationFeature, TranscriptTree

def test_variation_feature_initialization():
    """Ensure Python can instantiate the Rust struct and read its fields."""
    variant = VariationFeature("chr1", 15000, "A", "T")
    
    assert variant.chrom == "chr1"
    assert variant.pos == 15000
    assert variant.ref_allele == "A"
    assert variant.alt_allele == "T"

def test_calculate_consequence_logic():
    """Verify Rust's biological math returns the correct Sequence Ontology terms."""
    # SNP (Same length)
    snp = VariationFeature("chr1", 100, "A", "T")
    assert snp.calculate_consequence() == "missense_variant"

    # Deletion (Ref is longer than Alt)
    deletion = VariationFeature("chr1", 100, "AG", "A")
    assert deletion.calculate_consequence() == "frameshift_variant"

    # Insertion (Alt is longer than Ref)
    insertion = VariationFeature("chr1", 100, "A", "ATCG")
    assert insertion.calculate_consequence() == "inframe_insertion"

def test_interval_tree_overlaps():
    """Test the Rust rust-lapper interval tree."""
    tree = TranscriptTree()
    
    # Position 1500 is inside the 1000-2000 mock interval
    overlaps = tree.find_overlapping(1500)
    assert len(overlaps) == 1
    assert overlaps[0] == "GeneA"
    
    # Position 5000 is outside
    empty_overlaps = tree.find_overlapping(5000)
    assert len(empty_overlaps) == 0