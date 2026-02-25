import pysam
from vep_pyrs.vep_core import VariationFeature
from vep_pyrs.writer import VcfWriter

class ParallelRunner:
    def __init__(self, input_vcf, output_vcf):
        self.input_vcf = input_vcf
        self.output_vcf = output_vcf

    def run(self):
        writer = VcfWriter(self.input_vcf, self.output_vcf)
        # vcf_in = pysam.VariantFile(self.input_vcf)

        for record in writer.vcf_in:
            alt = record.alts[0] if record.alts else ""
            
            # 1. Call Rust to do the math
            var = VariationFeature(record.chrom, record.pos, record.ref, alt)
            consequence = var.calculate_consequence()
            
            # 2. Format CSQ string (Allele|Consequence|Transcript)
            csq_str = f"{alt}|{consequence}|ENST_mock"
            record.info['CSQ'] = csq_str
            
            # 3. Write to disk
            writer.write(record)
            
        print("Annotation complete.")