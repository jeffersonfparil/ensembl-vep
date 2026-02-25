import pysam

class VcfWriter:
    def __init__(self, input_path, output_path):
        self.vcf_in = pysam.VariantFile(input_path)
        self.header = self.vcf_in.header
        
        # Add the CSQ info header
        self.header.add_meta(
            key='INFO',
            items=[('ID', 'CSQ'), ('Number', '.'), ('Type', 'String'), 
                   ('Description', 'Format: Allele|Consequence|Transcript')]
        )
        self.vcf_out = pysam.VariantFile(output_path, 'w', header=self.header)

    def write(self, record):
        self.vcf_out.write(record)