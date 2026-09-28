"""Score saved Rust/Nora candidates against fixed behavioral checks.

Runs native code locally. Temporary directories isolate artifacts, not security.
"""
import argparse
import hashlib
import json
import pathlib
import subprocess
import tempfile
import time

ROOT = pathlib.Path(__file__).resolve().parents[1]
BENCH = ROOT / 'benchmarks/programming-rust'
TASKS = ('gcd', 'queue', 'interval', 'generic_queue', 'transform')
EXPECTED_TESTS = {'gcd': 3, 'queue': 2, 'interval': 1, 'generic_queue': 3, 'transform': 1}


def evaluate(task, candidate, compiler, timeout=10):
    started = time.monotonic()
    result = {'task': task, 'candidate': str(candidate), 'status': 'missing'}
    if not candidate.is_file():
        return result
    source = candidate.read_text()
    result.update(source_bytes=len(source.encode()), sha256=hashlib.sha256(source.encode()).hexdigest(),
                  language=candidate.suffix.lstrip('.'), rust_escape=any(line.strip() == '%%rust' for line in source.splitlines()))
    try:
        import tiktoken
        result['source_tokens_o200k_base'] = len(tiktoken.get_encoding('o200k_base').encode(source, disallowed_special=()))
    except ImportError:
        result['source_tokens_o200k_base'] = None
    try:
        with tempfile.TemporaryDirectory(prefix='nora-benchmark-') as tmp:
            work = pathlib.Path(tmp)
            if candidate.suffix == '.nora':
                if compiler is None:
                    raise ValueError('Nora candidates require --compiler')
                translated = subprocess.run([str(compiler), str(candidate.resolve())], capture_output=True, text=True, timeout=timeout, cwd=work)
                if translated.returncode:
                    result.update(status='nora_failed', diagnostics=translated.stderr[-8000:])
                    return result
                source = translated.stdout
            program = work / 'candidate.rs'
            program.write_text(source + '\n' + (BENCH / 'checks' / f'{task}.rs').read_text())
            binary = work / 'checks'
            compiled = subprocess.run(['rustc', '--edition=2024', '--test', str(program), '-o', str(binary)], capture_output=True, text=True, timeout=timeout, cwd=work)
            if compiled.returncode:
                result.update(status='compile_failed', diagnostics=compiled.stderr[-8000:])
                return result
            # Ensure candidate attributes cannot silently disable every check.
            listing = subprocess.run([str(binary), '--list', 'nora_bench'], capture_output=True, text=True, timeout=timeout, cwd=work)
            count = sum(line.startswith('nora_bench') and line.endswith(': test') for line in listing.stdout.splitlines())
            if listing.returncode or count != EXPECTED_TESTS[task]:
                result.update(status='test_failed', diagnostics='Expected benchmark checks were not discovered')
                return result
            tested = subprocess.run([str(binary), 'nora_bench', '--test-threads=1', '--include-ignored'], capture_output=True, text=True, timeout=timeout, cwd=work)
            result.update(status='pass' if tested.returncode == 0 else 'test_failed', diagnostics=(tested.stdout + tested.stderr)[-8000:])
    except subprocess.TimeoutExpired:
        result['status'] = 'timeout'
    finally:
        result['elapsed_seconds'] = round(time.monotonic() - started, 3)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--candidates', type=pathlib.Path, default=BENCH / 'reference')
    parser.add_argument('--language', choices=['rs', 'nora'], default='rs')
    parser.add_argument('--compiler', type=pathlib.Path, default=ROOT / 'target/debug/nora')
    parser.add_argument('--model', default='reference', help='Label only; this runner does not call a model')
    parser.add_argument('--report', type=pathlib.Path)
    args = parser.parse_args()
    results = [evaluate(task, args.candidates / f'{task}.{args.language}', args.compiler.resolve()) for task in TASKS]
    report = {'suite': 'programming-rust-v1', 'model': args.model, 'upstream_commit': 'ce1eb55d7ecf69450202587b11ad3ca171b875e3', 'results': results, 'passed': sum(r['status'] == 'pass' for r in results), 'total': len(TASKS)}
    text = json.dumps(report, indent=2)
    if args.report:
        args.report.parent.mkdir(parents=True, exist_ok=True)
        args.report.write_text(text + '\n')
    print(text)
    return 0 if report['passed'] == report['total'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
