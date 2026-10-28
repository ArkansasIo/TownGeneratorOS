import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).with_name("validate_geojson.py")


class GeoJsonValidatorTests(unittest.TestCase):
    def validate(self, payload):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "sample.geojson"
            path.write_text(json.dumps(payload), encoding="utf-8")
            return subprocess.run(
                [sys.executable, str(SCRIPT), str(path), "--max-invalid", "1"],
                capture_output=True, text=True, check=False,
            )

    def test_valid_feature_collection_passes(self):
        result = self.validate({"type":"FeatureCollection","features":[{"type":"Feature","properties":{},"geometry":{"type":"Point","coordinates":[-79.38,43.65]}}]})
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_out_of_range_coordinates_fail(self):
        result = self.validate({"type":"FeatureCollection","features":[{"type":"Feature","properties":{},"geometry":{"type":"Point","coordinates":[220,95]}}]})
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("invalid", (result.stdout + result.stderr).lower())

    def test_empty_collection_is_valid(self):
        result = self.validate({"type":"FeatureCollection","features":[]})
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)


if __name__ == "__main__":
    unittest.main()
