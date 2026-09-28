import pathlib
import tempfile
import unittest
from bench_ai_guide import score

GOOD = '''pub fn nora_offset(x:i64,delta:i64)->i64{x+delta}
pub fn nora_total(q:i64,price:i64,discount:i64)->i64{q*price-discount}
pub fn nora_label(a:i64,b:i64)->String{(a+b).to_string()}
pub fn nora_identity(x:String)->String{x}
pub fn nora_bucket(x:i64,width:i64)->i64{(x+3)/width}'''


class GuideScorerTests(unittest.TestCase):
    def test_correct_and_wrong_precedence(self):
        with tempfile.TemporaryDirectory() as tmp:
            work = pathlib.Path(tmp)
            self.assertEqual(score(GOOD, 'rs', work)['status'], 'pass')
            self.assertEqual(score(GOOD.replace('(x+3)/width', 'x+3/width'), 'rs', work)['status'], 'test_failed')
            self.assertEqual(score('invalid code', 'rs', work)['status'], 'compile_failed')
            self.assertEqual(score('#![cfg(any())]\n' + GOOD, 'rs', work)['status'], 'test_failed')


if __name__ == '__main__':
    unittest.main()
