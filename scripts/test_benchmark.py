import importlib.util
import pathlib
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("benchmark", ROOT / "scripts/benchmark.py")
benchmark = importlib.util.module_from_spec(spec)
spec.loader.exec_module(benchmark)


class BenchmarkTests(unittest.TestCase):
    def test_correct_reference_and_incorrect_candidate(self):
        reference = ROOT / 'benchmarks/programming-rust/reference/gcd.rs'
        self.assertEqual(benchmark.evaluate('gcd', reference, None)['status'], 'pass')
        with tempfile.TemporaryDirectory() as tmp:
            candidate = pathlib.Path(tmp) / 'gcd.rs'
            candidate.write_text('fn gcd(_: u64, _: u64) -> u64 { 1 }')
            self.assertEqual(benchmark.evaluate('gcd', candidate, None)['status'], 'test_failed')
            candidate.write_text('not Rust')
            self.assertEqual(benchmark.evaluate('gcd', candidate, None)['status'], 'compile_failed')
            self.assertEqual(benchmark.evaluate('gcd', candidate.with_name('missing.rs'), None)['status'], 'missing')

    def test_timeout_is_failure(self):
        with tempfile.TemporaryDirectory() as tmp:
            candidate = pathlib.Path(tmp) / 'gcd.rs'
            candidate.write_text('fn gcd(_: u64, _: u64) -> u64 { loop {} }')
            self.assertEqual(benchmark.evaluate('gcd', candidate, None, timeout=1)['status'], 'timeout')

class MoreBenchmarkTests(unittest.TestCase):
    def test_suite_rejects_broken_semantics(self):
        mutations = {
            'queue': ('self.older.reverse();', ''),
            'generic_queue': ('self.older.reverse();', ''),
            'interval': ('Some(Ordering::Less)', 'Some(Ordering::Greater)'),
            'transform': ('q * p - discount', 'q * p + discount'),
        }
        for task, (before, after) in mutations.items():
            source = (benchmark.BENCH / 'reference' / f'{task}.rs').read_text()
            self.assertIn(before, source)
            with tempfile.TemporaryDirectory() as tmp:
                candidate = pathlib.Path(tmp) / f'{task}.rs'
                candidate.write_text(source.replace(before, after))
                self.assertEqual(benchmark.evaluate(task, candidate, None)['status'], 'test_failed', task)

    def test_disabled_checks_are_failure(self):
        with tempfile.TemporaryDirectory() as tmp:
            candidate = pathlib.Path(tmp) / 'gcd.rs'
            candidate.write_text('#![cfg(any())]\nfn gcd(_: u64, _: u64) -> u64 { 1 }')
            self.assertEqual(benchmark.evaluate('gcd', candidate, None)['status'], 'test_failed')


if __name__ == '__main__':
    unittest.main()
