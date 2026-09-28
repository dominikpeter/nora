"""Live, one-shot Codex experiment. Calls a model and executes its output locally."""
import argparse
import hashlib
import json
import pathlib
import subprocess
import tempfile
import time

ROOT = pathlib.Path(__file__).resolve().parents[1]
FIXTURES = ROOT / 'benchmarks/ai-guide'


def score(source, language, work):
    path = work / ('candidate.' + language)
    path.write_text(source)
    try:
        if language == 'nora':
            compiled = subprocess.run([str(ROOT / 'target/debug/nora'), str(path)], capture_output=True, text=True, timeout=10)
            if compiled.returncode:
                return {'status': 'nora_failed', 'diagnostics': compiled.stderr}
            source = compiled.stdout
        rust = work / 'checked.rs'
        rust.write_text(source + '\n' + (FIXTURES / 'checks.rs').read_text())
        binary = work / 'checks'
        compiled = subprocess.run(['rustc', '--edition=2024', '--test', str(rust), '-o', str(binary)], capture_output=True, text=True, timeout=20)
        if compiled.returncode:
            return {'status': 'compile_failed', 'diagnostics': compiled.stderr}
        listed = subprocess.run([str(binary), '--list', 'guide_checks::'], capture_output=True, text=True, timeout=10)
        count = sum(line.startswith('guide_checks::') and line.endswith(': test') for line in listed.stdout.splitlines())
        if listed.returncode or count != 5:
            return {'status': 'test_failed', 'diagnostics': 'Expected five evaluator checks'}
        tested = subprocess.run([str(binary), 'guide_checks::', '--include-ignored'], capture_output=True, text=True, timeout=10)
        return {'status': 'pass' if tested.returncode == 0 else 'test_failed', 'diagnostics': tested.stdout + tested.stderr}
    except subprocess.TimeoutExpired:
        return {'status': 'timeout'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--live', action='store_true', help='Required: authorize live model calls')
    parser.add_argument('--guide', type=pathlib.Path, help='Alternative guide for an explicit experiment')
    parser.add_argument('--condition', choices=['all', 'nora_with_guide'], default='all')
    parser.add_argument('--report-dir', type=pathlib.Path, default=ROOT / 'target/ai-guide')
    args = parser.parse_args()
    if not args.live:
        parser.error('Use --live to run three model calls; for offline tests use just test')
    subprocess.run(['cargo', 'build', '--quiet'], cwd=ROOT, check=True)
    import tiktoken
    encoder = tiktoken.get_encoding('o200k_base')
    args.report_dir.mkdir(parents=True, exist_ok=True)
    task = (FIXTURES / 'task.md').read_text()
    guide = args.guide.read_text() if args.guide else subprocess.check_output([str(ROOT / 'target/debug/nora'), '--ai-reference'], text=True)
    report = {'kind': 'live_model_pilot', 'trials_per_condition': 1,
              'cli': subprocess.check_output(['codex', '--version'], text=True).strip(),
              'model_selection': 'Codex default with user config ignored',
              'task_sha256': hashlib.sha256(task.encode()).hexdigest(),
              'guide_sha256': hashlib.sha256(guide.encode()).hexdigest(),
              'guide_tokens_o200k_base': len(encoder.encode(guide)), 'results': []}
    for condition, language in [('rust', 'rs'), ('nora_no_guide', 'nora'), ('nora_with_guide', 'nora')]:
        if args.condition != 'all' and condition != args.condition:
            continue
        prompt = ('Write Rust 2024 source.\n' if language == 'rs' else 'Write Nora v0.1 source that compiles to the required Rust interfaces.\n')
        if condition == 'nora_with_guide':
            prompt += guide + '\n'
        prompt += task
        (args.report_dir / f'{condition}.prompt.txt').write_text(prompt)
        print(f'Running {condition}...', flush=True)
        start = time.monotonic()
        with tempfile.TemporaryDirectory(prefix='nora-model-') as tmp:
            work = pathlib.Path(tmp)
            output = work / 'response.txt'
            command = ['codex', 'exec', '--ignore-user-config', '--ephemeral', '--skip-git-repo-check', '--sandbox', 'read-only', '--json', '-C', str(work), '-o', str(output), '-']
            try:
                run = subprocess.run(command, input=prompt, capture_output=True, text=True, timeout=180, cwd=work)
                (args.report_dir / f'{condition}.events.jsonl').write_text(run.stdout)
                (args.report_dir / f'{condition}.stderr.txt').write_text(run.stderr)
                events = []
                for line in run.stdout.splitlines():
                    try:
                        events.append(json.loads(line))
                    except json.JSONDecodeError:
                        pass
                usage = [e['usage'] for e in events if e.get('type') == 'turn.completed' and 'usage' in e]
                tools = [e for e in events if e.get('type') == 'item.completed' and e.get('item', {}).get('type') not in ('agent_message', 'reasoning')]
                source = output.read_text() if output.exists() else ''
                (args.report_dir / f'{condition}.{language}').write_text(source)
                result = {'condition': condition, 'prompt_tokens_o200k_base': len(encoder.encode(prompt)),
                          'source_tokens_o200k_base': len(encoder.encode(source, disallowed_special=())),
                          'usage': usage, 'tool_events': len(tools),
                          'rust_escape': any(line.strip() == '%%rust' for line in source.splitlines())}
                if run.returncode or not source:
                    result.update(status='model_failed', diagnostics=run.stderr[-4000:])
                elif tools:
                    result.update(status='excluded_tool_use', diagnostics='Prompt-only experiment excludes tool use')
                else:
                    result.update(score(source, language, work))
            except subprocess.TimeoutExpired:
                result = {'condition': condition, 'status': 'model_timeout'}
        result['elapsed_seconds'] = round(time.monotonic() - start, 2)
        report['results'].append(result)
        (args.report_dir / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
        print(f"{condition}: {result['status']}", flush=True)
    print(f'Report: {args.report_dir / "report.json"}')
    return 0 if all(r['status'] == 'pass' for r in report['results']) else 1


if __name__ == '__main__':
    raise SystemExit(main())
