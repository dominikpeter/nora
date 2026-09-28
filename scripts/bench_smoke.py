"""Run reference Rust and Nora compatibility fixtures; no model is called."""
import json
import pathlib
import subprocess
import tempfile
from benchmark import BENCH, ROOT, TASKS, evaluate

subprocess.run(['cargo', 'build', '--quiet'], cwd=ROOT, check=True)
compiler = ROOT / 'target/debug/nora'
rows = []
with tempfile.TemporaryDirectory(prefix='nora-fixtures-') as tmp:
    for task in TASKS:
        reference = BENCH / 'reference' / f'{task}.rs'
        baseline = evaluate(task, reference, compiler)
        candidate = pathlib.Path(tmp) / f'{task}.nora'
        candidate.write_text('price(q,p,discount:i)=q*p-discount\nlabel(x:i)=$x\n' if task == 'transform' else '%%rust\n' + reference.read_text())
        translated = evaluate(task, candidate, compiler)
        rows.append({'task': task, 'rust': baseline, 'nora': translated,
                     'coverage': 'compact' if task == 'transform' else 'rust_passthrough'})
report = {'kind': 'fixture_smoke_not_model_benchmark', 'results': rows}
path = ROOT / 'target/benchmarks/smoke.json'
path.parent.mkdir(parents=True, exist_ok=True)
path.write_text(json.dumps(report, indent=2) + '\n')
for row in rows:
    print(f"{row['task']:16} Rust={row['rust']['status']:12} Nora={row['nora']['status']:12} {row['coverage']}")
print(f'Report: {path}')
raise SystemExit(0 if all(r[k]['status'] == 'pass' for r in rows for k in ('rust', 'nora')) else 1)
