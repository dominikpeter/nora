"""Paired, repeated live Rust/Nora trials on larger programs."""
import argparse
import hashlib
import json
import pathlib
import subprocess
import tempfile
import time
from bench_ai_guide import ROOT, score

SUITE = ROOT / 'benchmarks/larger'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--live', action='store_true')
    parser.add_argument('--model', default='gpt-6-astra')
    parser.add_argument('--trials', type=int, default=2)
    parser.add_argument('--report-dir', type=pathlib.Path, default=ROOT / 'target/larger-benchmark')
    args = parser.parse_args()
    if not args.live or args.trials < 1:
        parser.error('--live and a positive trial count are required')
    import tiktoken
    encoder = tiktoken.get_encoding('o200k_base')
    count = lambda s: len(encoder.encode(s, disallowed_special=()))
    subprocess.run(['cargo', 'build', '--quiet'], cwd=ROOT, check=True)
    guide = (SUITE / 'guide.txt').read_text()
    args.report_dir.mkdir(parents=True, exist_ok=True)
    report = {'kind': 'larger_paired_pilot', 'model': args.model, 'effort': 'medium',
              'cli': subprocess.check_output(['codex', '--version'], text=True).strip(),
              'trials_per_condition_per_task': args.trials, 'repair_budget': 0,
              'guide_tokens_o200k_base': count(guide), 'guide_sha256': hashlib.sha256(guide.encode()).hexdigest(),
              'results': []}
    (args.report_dir / 'guide.txt').write_text(guide)
    for task in ('invoice', 'inventory'):
        spec = (SUITE / 'tasks' / f'{task}.md').read_text()
        checks = SUITE / 'checks' / f'{task}.rs'
        for trial in range(args.trials):
            for language in (('rs', 'nora') if trial % 2 == 0 else ('nora', 'rs')):
                key = f'{task}-{trial+1}-{language}'
                prompt = ('Write Rust 2024.\n' if language == 'rs' else 'Write Nora v0.1.\n' + guide + '\n') + spec
                (args.report_dir / f'{key}.prompt.txt').write_text(prompt)
                print(f'Running {key}...', flush=True)
                start = time.monotonic()
                row = {'task': task, 'trial': trial+1, 'language': language,
                       'task_sha256': hashlib.sha256(spec.encode()).hexdigest(),
                       'checks_sha256': hashlib.sha256(checks.read_bytes()).hexdigest(),
                       'prompt_tokens_o200k_base': count(prompt)}
                with tempfile.TemporaryDirectory(prefix='nora-larger-') as tmp:
                    work = pathlib.Path(tmp)
                    output = work / 'response.txt'
                    cmd = ['codex', 'exec', '--ignore-user-config', '--model', args.model,
                           '-c', 'model_reasoning_effort="medium"', '--ephemeral', '--skip-git-repo-check',
                           '--sandbox', 'read-only', '--json', '-C', str(work), '-o', str(output), '-']
                    try:
                        run = subprocess.run(cmd, input=prompt, capture_output=True, text=True, timeout=240, cwd=work)
                        (args.report_dir / f'{key}.events.jsonl').write_text(run.stdout)
                        (args.report_dir / f'{key}.stderr.txt').write_text(run.stderr)
                        events = []
                        for line in run.stdout.splitlines():
                            try:
                                events.append(json.loads(line))
                            except json.JSONDecodeError:
                                pass
                        usage = [e['usage'] for e in events if e.get('type') == 'turn.completed' and 'usage' in e]
                        tools = [e for e in events if e.get('type') == 'item.completed' and e.get('item', {}).get('type') not in ('agent_message', 'reasoning')]
                        source = output.read_text() if output.exists() else ''
                        (args.report_dir / f'{key}.{language}').write_text(source)
                        head = source.split('%%rust',1)[0] if language == 'nora' else ''
                        row.update(usage=usage, source_tokens_o200k_base=count(source),
                                   prompt_plus_source_tokens=count(prompt)+count(source), tool_events=len(tools),
                                   rust_escape=any(line.strip()=='%%rust' for line in source.splitlines()),
                                   compact_source_tokens_o200k_base=count(head) if language=='nora' else 0)
                        if run.returncode or not source:
                            row.update(status='model_failed', diagnostics=run.stderr[-4000:])
                        elif tools:
                            row.update(status='excluded_tool_use', diagnostics='Tool use excluded in one-shot benchmark')
                        else:
                            row.update(score(source, language, work, checks))
                    except subprocess.TimeoutExpired:
                        row['status'] = 'model_timeout'
                row['elapsed_seconds'] = round(time.monotonic()-start,2)
                report['results'].append(row)
                (args.report_dir / 'report.json').write_text(json.dumps(report,indent=2)+'\n')
                print(f"{key}: {row['status']}; source={row.get('source_tokens_o200k_base')}; prompt+source={row.get('prompt_plus_source_tokens')}", flush=True)
    print(f'Report: {args.report_dir / "report.json"}')
    return 0 if all(r['status']=='pass' for r in report['results']) else 1


if __name__ == '__main__':
    raise SystemExit(main())
