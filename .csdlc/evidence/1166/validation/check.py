"""Actual retained episode integrity checks; no network or subjective listening claim."""
from pathlib import Path
import hashlib,json,re,struct,subprocess,wave
root=Path(__file__).resolve().parents[4]
b=root/'demos/podcast/editorial/1166-episode-1-introduction';d=b/'audio-candidate'
def load(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def check_file(p,v):
 assert p.is_file(),p
 assert p.stat().st_size==v['bytes'] and sha(p)==v['sha256'],p
for name,v in load(b/'preserved-candidate.json')['files'].items():check_file(root/name,v)
for name,v in load(d/'storage-manifest.json')['files'].items():check_file(d/name,v)
m=load(b/'own-words/manifest.json');p=load(d/'production.json');e=load(d/'episode.json');r=load(d/'rss-enclosure.json')
assert m['human_approved'] and m['audio_rendered']
assert m['script_sha256']==sha(b/'own-words/script.md')==p['script_sha256']==e['script_sha256']
turns=[load(f) for f in sorted((b/'own-words').glob('[0-9][0-9]-*.json'))];assert len(turns)==18
script=(b/'own-words/script.md').read_text();spoken=re.findall(r'### (ChatGPT|Gemini|Claude)\n\n(.*?)(?=\n### |\Z)',script,re.S)
assert [(t['speaker'],t['text']) for t in turns]==[(s,t.strip()) for s,t in spoken]
for t in turns:
 assert hashlib.sha256(t['text'].encode()).hexdigest()==t['text_sha256']
 f=b/'own-words'/t.get('prompt_file',f'{t["turn"]:02d}-prompt.txt');s=f.read_text()
 if t.get('prompt_file_encoding')!='exact_prompt_bytes':s=s.removesuffix('\n')
 assert hashlib.sha256(s.encode()).hexdigest()==t['prompt_sha256'],f
 assert t['completion']['finish_reason'] in ['completed','STOP','end_turn']
assert sum(t['words'] for t in turns)==m['spoken_words']
assert sum(len(re.findall(r"\b[\w]+(?:[’'-][\w]+)*\b",t['text'])) for t in turns)==m['spoken_words']
for name,v in p['files'].items():check_file(d/name,v)
with wave.open(str(d/'episode.wav')) as w:
 assert (w.getnchannels(),w.getsampwidth(),w.getframerate())==(1,2,24000)
 duration=w.getnframes()/w.getframerate();assert 540<=duration<=660
 assert abs(duration*p['tempo_factor']-p['natural_render_seconds'])<0.01
assert 0.85<=p['tempo_factor']<=1 and abs(e['audio_duration_seconds']-duration)<0.1
assert e['audio_duration_seconds']==p['duration_seconds']==r['duration_seconds']
assert e['audio_sha256']==r['sha256']==sha(d/'episode.mp3')
assert e['audio_bytes']==r['length']==(d/'episode.mp3').stat().st_size
cover=(b/'cognitive_stack_cover_3000.png').read_bytes();assert cover[:8]==b'\x89PNG\r\n\x1a\n'
assert struct.unpack('>II',cover[16:24])==(3000,3000)
assert hashlib.sha256(cover).hexdigest()==e['artwork_sha256']==e['embedded_artwork_sha256']
mp3=(d/'episode.mp3').read_bytes();assert mp3[:3]==b'ID3'
taglen=sum(c<<(7*(3-i)) for i,c in enumerate(mp3[6:10]));tag=mp3[10:10+taglen];assert b'APIC' in tag and cover in tag
for t,segment in zip(turns,p['segments']):assert t['text_sha256']==segment['source_text_sha256'] and t['speaker']==segment['speaker']
chapters=load(d/'chapters.json')['chapters'];assert len(chapters)==len(e['chapters'])==5
for c,ec in zip(chapters,e['chapters']):
 assert c['startTime']==ec['start_seconds']==p['segments'][ec['source_turn']-1]['start_seconds'] and c['title']==ec['title']
assert e['publication_date'] is None and not e['public_release_authorized'] and not e['human_audio_approved']
assert r['url'] is None and r['publication_date'] is None
l=load(d/'loudness-final.json');assert -17<=float(l['input_i'])<=-15 and float(l['input_tp'])<=-1
assert load(d/'transcript-comparison.json')['similarity']>0.98
observed=(d/'transcription-observed.txt').read_text().lower()
assert "let's introduce ourselves" in observed
assert 'deepseek' in observed.replace(' ', '') and 'special guest' in observed
trans=(d/'transcript.md').read_text();ts=re.findall(r'### (ChatGPT|Gemini|Claude)\n\n(.*?)(?=\n### |\Z)',trans,re.S)
expected=[(t['speaker'],t['text'].replace('a digital stalemate, where neither','a digital stalemate, when neither') if t['turn']==11 else t['text']) for t in turns]
assert [(s,t.strip()) for s,t in ts]==expected
subprocess.run(['git','diff','--check'],cwd=root,check=True)
subprocess.run(['git','diff','--cached','--check'],cwd=root,check=True)
print('PASS: 77 preserved files, media/hash/artwork, 18 authored turns, transcript, measured chapters and publication hold; human listening pending')
