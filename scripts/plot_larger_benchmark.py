"""Rebuild larger-program result summary and README graph from recorded trials."""
import html
import json
import statistics
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
report = json.loads((ROOT / 'benchmarks/larger/results/report.json').read_text())
summary = []
for task in ('invoice', 'inventory'):
    for lang in ('rs', 'nora'):
        rows = [r for r in report['results'] if r['task']==task and r['language']==lang]
        usage = [sum(u['input_tokens']+u['output_tokens'] for u in r['usage']) for r in rows]
        summary.append({'task':task, 'language':lang, 'passed':sum(r['status']=='pass' for r in rows),
                        'trials':len(rows),
                        'mean_source_tokens':statistics.mean(r['source_tokens_o200k_base'] for r in rows),
                        'mean_prompt_plus_source_tokens':statistics.mean(r['prompt_plus_source_tokens'] for r in rows),
                        'mean_provider_input_plus_output_tokens':statistics.mean(usage),
                        'mean_reasoning_output_tokens':statistics.mean(sum(u.get('reasoning_output_tokens',0) for u in r['usage']) for r in rows)})
(ROOT / 'benchmarks/larger/results/summary.json').write_text(json.dumps(summary,indent=2)+'\n')
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="900" height="390" viewBox="0 0 900 390" role="img" aria-label="Paired larger-program benchmark: mean local prompt plus source tokens, including the Nora instruction.">','<rect width="900" height="390" rx="16" fill="#101820"/>','<g font-family="Arial, sans-serif" fill="#e8edf2">']
def text(x,y,s,size=14,color='#e8edf2'):
    svg.append(f'<text x="{x}" y="{y}" font-size="{size}" fill="{color}">{html.escape(str(s))}</text>')
text(30,40,'Larger programs · teaching overhead included',25)
text(30,66,'Mean local prompt + source tokens · 2 trials per language per task',14,'#b8c5d1')
scale=500/max(r['mean_prompt_plus_source_tokens'] for r in summary)
for index,row in enumerate(summary):
    y=104+index*47
    value=row['mean_prompt_plus_source_tokens']
    text(30,y+20,f"{row['task']} / {'Rust' if row['language']=='rs' else 'Nora'}")
    color='#799bc7' if row['language']=='rs' else '#45d9ad'
    svg.append(f'<rect x="195" y="{y}" width="{value*scale:.2f}" height="28" rx="4" fill="{color}"/>')
    text(205+value*scale,y+20,f"{value:g} · {row['passed']}/{row['trials']} pass")
text(30,330,'gpt-6-astra · medium · 103-token Nora guide · no repairs',14,'#b8c5d1')
text(30,354,'Local counts exclude reasoning/system context; provider totals are reported separately.',13,'#b8c5d1')
text(30,375,'Invoice uses compact helpers + Rust; inventory requires Rust passthrough. Small pilot.',13,'#b8c5d1')
svg.append('</g></svg>')
(ROOT/'docs/images/larger-benchmark.svg').write_text('\n'.join(svg)+'\n')
print(json.dumps(summary,indent=2))
