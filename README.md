# ensembl-vep

A draft re-implementation using a Rust-Python hybrid approach

```shell
ensemble-vep/
├── Cargo.toml                # Rust dependencies (pyo3, noodles, rayon)
├── pyproject.toml            # Python build config (maturin)
├──── src/                 # RUST CORE
│   ├── lib.rs                # PyO3 module bindings
│   ├── cache.rs              # Tabix/FASTA offline reading logic
│   ├── consequences.rs       # SO term calculation & Codon math
│   └── intervals.rs          # Fast Lapper interval trees
└── vep_pyrs/                 # PYTHON ORCHESTRATOR
    ├── __init__.py
    ├── cache_manager.py      # Downloads and verifies databases
    ├── parallel_runner.py    # Multiprocessing VCF chunking
    ├── writer.py             # VCF/JSON formatting and Headers
    └── cli.py                # CLI
```

```mermaid
flowchart TB
    %% External Files
    subgraph Storage [File System]
        InVCF[(Input VCF)]
        OutVCF[(Annotated VCF)]
        Cache[(Genomic Cache .gz & .tbi)]
    end

    %% Python Layer (I/O and Orchestration)
    subgraph PythonLayer [Python: Orchestration & I/O]
        CLI(Command Line)
        PySam[VCF Parser pysam]
        MultiProc[[Multiprocessing]]
        Writer[VCF Formatter]
    end

    %% Rust Layer (Performance and Math)
    subgraph RustLayer [Rust: High-Performance Core]
        PyO3{{PyO3 Bridge}}
        Tabix[Tabix Cache Reader]
        Trees[Interval Trees]
        Math[Consequence & Codon Math]
    end

    %% Execution Flow
    CLI --> MultiProc
    InVCF -->|Read Variants| PySam
    PySam -->|Feed Chunks| MultiProc
    
    %% The Language Bridge
    MultiProc ==>|Passes Variant Data| PyO3
    PyO3 ==>|Returns Annotation Strings| Writer
    
    %% Rust Internals
    PyO3 --> Trees
    PyO3 --> Math
    PyO3 --> Tabix
    
    Tabix <-->|Lightning-fast Reads| Cache
    
    %% Final Output
    Writer -->|Write Annotated Data| OutVCF
```

Dev stuff:

```shell
cargo install uv

cd ensembl-vep

uv sync # installs maturin, pytest, etc
uv tool run maturin develop # use maturin as a standalone tool to develop the package including Rust library compilation
uv run pytest tests/ -v # use pytest as part of the whole python project


```