import tempfile
import unittest
from pathlib import Path
from run_gate import reserve

class Recording(unittest.TestCase):
    def test_existing_evidence_is_unchanged_and_no_partial_new_record_created(self):
        with tempfile.TemporaryDirectory() as directory:
            first=Path(directory)/'manifest.json'
            existing=Path(directory)/'record.jsonl'
            existing.write_bytes(b'original evidence\n')
            with self.assertRaises(FileExistsError):
                reserve([first,existing])
            self.assertFalse(first.exists())
            self.assertEqual(existing.read_bytes(),b'original evidence\n')
