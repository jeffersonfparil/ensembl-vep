import os

class CacheManager:
    def __init__(self, version="115"):
        self.cache_dir = os.path.expanduser(f"~/.hybrid_vep_cache/{version}")
        self.gff = os.path.join(self.cache_dir, "transcripts.gff.gz")

    def ensure_cache_exists(self):
        if not os.path.exists(self.gff):
            print(f"Cache missing. In production, this downloads to {self.gff}")
            # Mocking existence for the scaffold
            os.makedirs(self.cache_dir, exist_ok=True)
            with open(self.gff, 'w') as f: f.write("mock")
        return self.gff