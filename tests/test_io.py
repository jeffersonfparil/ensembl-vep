# tests/test_vcf_io.py
import pysam
import pytest
from vep_pyrs.writer import VcfWriter

@pytest.fixture
def dummy_vcf(tmp_path):
    """Creates a minimal valid VCF file for testing."""
    vcf_path = tmp_path / "test_input.vcf"
    
    # Write a minimal VCF header and one variant record
    content = (
        "##fileformat=VCFv4.2\n"
        "##contig=<ID=chr1,length=248956422>\n"
        "#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\n"
        "chr1\t10000\t.\tA\tT\t.\tPASS\t.\n"
    )
    vcf_path.write_text(content)
    return str(vcf_path)

def test_vcf_writer_injects_csq_header(dummy_vcf, tmp_path):
    """Ensure the VcfWriter adds the ##INFO=<ID=CSQ...> header."""
    out_path = str(tmp_path / "test_output.vcf")
    
    # Initialize the writer
    writer = VcfWriter(dummy_vcf, out_path)
    
    # Check that CSQ is now in the header
    assert "CSQ" in writer.header.info
    assert "Allele|Consequence|Transcript" in writer.header.info["CSQ"].description

def test_vcf_writer_saves_record(dummy_vcf, tmp_path):
    """Ensure we can append the CSQ string and save the VCF."""
    out_path = str(tmp_path / "test_output.vcf")
    
    # Initialize the writer (this automatically injects CSQ into writer.vcf_in.header)
    writer = VcfWriter(dummy_vcf, out_path)
    
    # Fetch the record using the writer's internal reader!
    record = next(writer.vcf_in)
    
    # Now pysam knows what CSQ is, and this will work perfectly
    record.info["CSQ"] = "T|missense_variant|ENST_mock"
    writer.write(record)
    
    # Close files
    writer.vcf_in.close()
    writer.vcf_out.close()
    
    # Re-open the output to verify it wrote correctly
    verify_vcf = pysam.VariantFile(out_path)
    written_record = next(verify_vcf)
    
    assert "CSQ" in written_record.info
    assert written_record.info["CSQ"][0] == "T|missense_variant|ENST_mock" 
    # Note: pysam parses INFO strings as tuples, so we check index [0]