# tests/test_cli.py
import sys
import pytest
from unittest.mock import patch
from vep_pyrs.cli import run_vep, run_recoder, run_filter

@pytest.fixture
def dummy_vcf(tmp_path):
    """Creates a minimal valid VCF file for testing the CLI."""
    vcf_path = tmp_path / "test_input.vcf"
    content = (
        "##fileformat=VCFv4.2\n"
        "##contig=<ID=chr1,length=248956422>\n"  # <-- CRITICAL FIX HERE
        "#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\n"
        "chr1\t10000\t.\tA\tT\t.\tPASS\t.\n"
    )
    vcf_path.write_text(content)
    return str(vcf_path)

def test_run_vep_success(dummy_vcf, tmp_path, monkeypatch):
    """Test that the CLI successfully parses arguments and runs the pipeline."""
    out_path = str(tmp_path / "test_output.vcf")
    
    # Mock the cache directory so we don't write to your real home folder
    monkeypatch.setattr("vep_pyrs.cache_manager.os.path.expanduser", lambda x: str(tmp_path / "cache"))
    
    test_args = ["vep_pyrs", "-i", dummy_vcf, "-o", out_path]
    
    with patch.object(sys, 'argv', test_args):
        run_vep()
        
    assert (tmp_path / "test_output.vcf").exists()

def test_run_vep_missing_args():
    """Test that missing required arguments causes argparse to exit gracefully."""
    test_args = ["vep_pyrs"] 
    with patch.object(sys, 'argv', test_args):
        with pytest.raises(SystemExit) as excinfo:
            run_vep()
        assert excinfo.value.code == 2

def test_run_recoder_stub(capsys):
    """Test the recoder CLI stub executes and prints expected output."""
    test_args = ["vep_pyrs_recoder"]
    with patch.object(sys, 'argv', test_args):
        run_recoder()
        
    captured = capsys.readouterr()
    assert "Running variant recoder" in captured.out