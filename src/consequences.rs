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
    // List of sequence ontology (SO) terms to define
    // 1.	transcript_ablation:	A feature ablation whereby the deleted region includes a transcript feature
    // 2.	splice_acceptor_variant:	A splice variant that changes the 2 base region at the 3' end of an intron
    // 3.	splice_donor_variant:	A splice variant that changes the 2 base region at the 5' end of an intron
    // 4.	stop_gained:	A sequence variant whereby at least one base of a codon is changed, resulting in a premature stop codon, leading to a shortened transcript
    // 5.	frameshift_variant:	A sequence variant which causes a disruption of the translational reading frame, because the number of nucleotides inserted or deleted is not a multiple of three
    // 6.	stop_lost:	A sequence variant where at least one base of the terminator codon (stop) is changed, resulting in an elongated transcript
    // 7.	start_lost:	A codon variant that changes at least one base of the canonical start codon
    // 8.	transcript_amplification:	A feature amplification of a region containing a transcript
    // 9.	feature_elongation:	A sequence variant that causes the extension of a genomic feature, with regard to the reference sequence
    // 10.	feature_truncation:	A sequence variant that causes the reduction of a genomic feature, with regard to the reference sequence
    // 11.	inframe_insertion:	An inframe non synonymous variant that inserts bases into in the coding sequence
    // 12.	inframe_deletion:	An inframe non synonymous variant that deletes bases from the coding sequence
    // 13.	missense_variant:	A sequence variant, that changes one or more bases, resulting in a different amino acid sequence but where the length is preserved
    // 14.	protein_altering_variant:	A sequence_variant which is predicted to change the protein encoded in the coding sequence
    // 15.	splice_donor_5th_base_variant:	A sequence variant that causes a change at the 5th base pair after the start of the intron in the orientation of the transcript
    // 16.	splice_region_variant:	A sequence variant in which a change has occurred within the region of the splice site, either within 1-3 bases of the exon or 3-8 bases of the intron
    // 17.	splice_donor_region_variant:	A sequence variant that falls in the region between the 3rd and 6th base after splice junction (5' end of intron)
    // 18.	splice_polypyrimidine_tract_variant:	A sequence variant that falls in the polypyrimidine tract at 3' end of intron between 17 and 3 bases from the end (acceptor -3 to acceptor -17)
    // 19.	incomplete_terminal_codon_variant:	A sequence variant where at least one base of the final codon of an incompletely annotated transcript is changed
    // 20.	start_retained_variant:	A sequence variant where at least one base in the start codon is changed, but the start remains
    // 21.	stop_retained_variant:	A sequence variant where at least one base in the terminator codon is changed, but the terminator remains
    // 22.	synonymous_variant:	A sequence variant where there is no resulting change to the encoded amino acid
    // 23.	coding_sequence_variant:	A sequence variant that changes the coding sequence
    // 24.	mature_miRNA_variant:	A transcript variant located with the sequence of the mature miRNA
    // 25.	5_prime_UTR_variant:	A UTR variant of the 5' UTR
    // 26.	3_prime_UTR_variant:	A UTR variant of the 3' UTR
    // 27.	non_coding_transcript_exon_variant:	A sequence variant that changes non-coding exon sequence in a non-coding transcript
    // 28.	intron_variant:	A transcript variant occurring within an intron
    // 29.	NMD_transcript_variant:	A variant in a transcript that is the target of NMD
    // 30.	non_coding_transcript_variant:	A transcript variant of a non coding RNA gene
    // 31.	coding_transcript_variant:	A transcript variant of a protein coding gene
    // 32.	upstream_gene_variant:	A sequence variant located 5' of a gene
    // 33.	downstream_gene_variant:	A sequence variant located 3' of a gene
    // 34.	TFBS_ablation:	A feature ablation whereby the deleted region includes a transcription factor binding site
    // 35.	TFBS_amplification:	A feature amplification of a region containing a transcription factor binding site
    // 36.	TF_binding_site_variant:	A sequence variant located within a transcription factor binding site
    // 37.	regulatory_region_ablation:	A feature ablation whereby the deleted region includes a regulatory region
    // 38.	regulatory_region_amplification:	A feature amplification of a region containing a regulatory region
    // 39.	regulatory_region_variant:	A sequence variant located within a regulatory region
    // 40.	intergenic_variant:	A sequence variant located in the intergenic region, between genes
    // 41.	sequence_variant:	A sequence_variant is a non exact copy of a sequence_feature or genome exhibiting one or more sequence_alteration
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