import pathlib
import tempfile
import unittest
from bench_ai_guide import ROOT, score

SUITE = ROOT / 'benchmarks/larger'

class LargerChecks(unittest.TestCase):
    def test_references_and_mutations(self):
        for task, before, after in [('invoice', 'total-paid', 'total+paid'), ('inventory', 'row.quantity<threshold', 'row.quantity<=threshold')]:
            source = (SUITE / 'reference' / f'{task}.rs').read_text()
            self.assertIn(before, source)
            with tempfile.TemporaryDirectory() as tmp:
                work = pathlib.Path(tmp)
                checks = SUITE / 'checks' / f'{task}.rs'
                self.assertEqual(score(source, 'rs', work, checks)['status'], 'pass', task)
                self.assertEqual(score(source.replace(before, after), 'rs', work, checks)['status'], 'test_failed', task)

if __name__ == '__main__':
    unittest.main()
