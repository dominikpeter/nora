"""Render the committed live-pilot results as SVG; no model calls required."""
import html
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
report = json.loads((ROOT / 'benchmarks/ai-guide/results/initial/report.json').read_text())
rows = report['results']
primer = json.loads((ROOT / 'benchmarks/ai-guide/results/primer/report.json').read_text())
rows += [dict(primer['results'][0], condition='nora_primer')]
labels = {'rust': 'Rust', 'nora_no_guide': 'Nora · no guide', 'nora_with_guide': 'Nora · full guide', 'nora_primer': 'Nora · short primer'}
svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="880" height="570" viewBox="0 0 880 570" role="img" aria-label="Live pilot: Rust passed at 131 source tokens; unguided Nora failed at 126; guided Nora passed at 55; short-primer Nora passed at 50. Guide overhead increases total prompt plus source tokens.">', '<rect width="880" height="570" rx="16" fill="#101820"/>', '<g font-family="Arial, sans-serif" fill="#e8edf2">']

def text(x, y, value, size=14, color='#e8edf2'):
    svg.append(f'<text x="{x}" y="{y}" font-size="{size}" fill="{color}">{html.escape(str(value))}</text>')

text(30, 40, 'Can an AI write Nora?', 26)
text(30, 66, 'Live Codex pilot · one trial per condition · five related functions', 14, '#b8c5d1')
for panel, (title, field, scale) in enumerate([
    ('Generated source tokens', 'source_tokens_o200k_base', 3.0),
    ('Prompt + source tokens (local text only)', None, 0.52),
]):
    top = 108 + panel * 205
    text(30, top, title, 17)
    for i, row in enumerate(rows):
        y = top + 25 + i * 37
        value = row[field] if field else row['prompt_tokens_o200k_base'] + row['source_tokens_o200k_base']
        color = '#45d9ad' if row['condition'] in ('nora_with_guide', 'nora_primer') else '#799bc7' if row['status']=='pass' else '#b08080'
        text(30, y+17, labels[row['condition']])
        svg.append(f'<rect x="205" y="{y}" width="{value*scale:.1f}" height="23" rx="4" fill="{color}"/>')
        text(215 + value*scale, y+17, f"{value} · {'5/5 passed' if row['status']=='pass' else 'compile failed'}")
text(30, 525, f"Guides: {report['guide_tokens_o200k_base']} → {primer['guide_tokens_o200k_base']} tokens. Tokenizer: o200k_base. Model identity not pinned.", 13, '#b8c5d1')
text(30, 548, 'Small onboarding experiment; not evidence of general Rust coverage or lower total agent cost.', 13, '#b8c5d1')
svg.append('</g></svg>')
path=ROOT/'docs/images/ai-guide-benchmark.svg'
path.parent.mkdir(parents=True, exist_ok=True)
path.write_text('\n'.join(svg)+'\n')
print(path)
