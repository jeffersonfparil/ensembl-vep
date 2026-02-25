import argparse
from vep_pyrs.parallel_runner import ParallelRunner
from vep_pyrs.cache_manager import CacheManager

def run_vep():
    parser = argparse.ArgumentParser(description="VEP CLI (experimental Rust-Python hybrid)")
    parser.add_argument("-i", "--input", required=True)
    parser.add_argument("-o", "--output", required=True)
    args = parser.parse_args()

    CacheManager().ensure_cache_exists()
    runner = ParallelRunner(args.input, args.output)
    runner.run()

def run_recoder():
    print("Running variant recoder (Scaffold)")

def run_filter():
    print("Running filter_vep (Scaffold)")