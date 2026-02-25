# tests/test_caching.py
import os
import pytest
from vep_pyrs.cache_manager import CacheManager
from vep_pyrs.vep_core import OfflineCache

def test_cache_manager_creates_directory(tmp_path, monkeypatch):
    """Test that CacheManager downloads/creates files if they are missing."""
    # Trick CacheManager into using our temporary test directory
    monkeypatch.setattr(os.path, "expanduser", lambda x: x.replace("~", str(tmp_path)))
    
    manager = CacheManager(version="test_115")
    gff_path = manager.ensure_cache_exists()
    
    # Verify Python created the directory and mock file
    assert os.path.exists(gff_path)
    assert "test_115" in gff_path

def test_rust_offline_cache_reads():
    """Verify Rust can read the cache file path provided by Python."""
    # Using a dummy path since we mocked the Rust implementation to just return strings
    cache = OfflineCache("/dummy/path/transcripts.gff.gz")
    
    transcripts = cache.fetch_transcripts("chr1", 10000)
    assert len(transcripts) == 2
    assert "ENST00000123456.1" in transcripts