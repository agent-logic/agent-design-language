"""Validate the prepared release against the immutable approved episode."""
import hashlib
import json
from pathlib import Path
import subprocess
import xml.etree.ElementTree as ET

root = Path(__file__).resolve().parents[4]
manifest = json.loads((root / 'demos/podcast/releases/episode-001/release.json').read_text())
ns = {'itunes': 'http://www.itunes.com/dtds/podcast-1.0.dtd'}
channel = ET.parse(root / manifest['feed']).getroot().find('channel')
item = channel.find('item')
assert len(channel.findall('item')) == 1
meta = json.loads((root / 'demos/podcast/editorial/1166-episode-1-introduction/audio-candidate/episode.json').read_text())
assert item.findtext('guid') == meta['guid'] == manifest['guid']
assert item.findtext('title') == meta['title']
assert item.findtext('description') == item.findtext('itunes:summary', namespaces=ns) == meta['summary']
assert item.findtext('itunes:duration', namespaces=ns) == '00:09:10'
assert item.find('pubDate') is None and manifest['publication_time'] is None
assert manifest['status'] == 'prepared_not_deployed'
for asset in manifest['assets']:
    payload = (root / asset['source']).read_bytes()
    assert payload == (root / asset['approved_source']).read_bytes()
    assert hashlib.sha256(payload).hexdigest() == asset['sha256']
    assert len(payload) == asset['bytes']
    assert all(url.startswith('https://agent-logic.ai/podcast/') for url in asset['public_urls'])
audio = manifest['assets'][0]
enclosure = item.find('enclosure')
assert enclosure.attrib == {'url': audio['public_urls'][0], 'length': str(audio['bytes']), 'type': 'audio/mpeg'}
assert audio['sha256'] == meta['audio_sha256']
import wave
with wave.open(str(root / manifest['archive_audio'])) as wav:
    assert abs(wav.getnframes() / wav.getframerate() - manifest['duration_seconds']) < 0.1
assert '_private' not in (root / manifest['feed']).read_text()
assert channel.findtext('link') == 'https://agent-logic.ai/podcast/'
for rel in ['demos/podcast/index.html', 'demos/podcast/episodes/meet-the-ai-coworkers/index.html']:
    assert (root / rel).read_bytes() == subprocess.check_output(['git', 'show', '2fbf1237abd4d5933b2dcb7ca60e54626b413420:' + rel], cwd=root)
assert not subprocess.check_output(['git','diff','2fbf1237abd4d5933b2dcb7ca60e54626b413420','--','demos/podcast/editorial/1166-episode-1-introduction'], cwd=root)
print('PASS: feed metadata, 3 exact asset copies, hashes/bytes, retained MP3 hash and measured WAV duration, unchanged webpage and approved source')

private = (root / 'demos/podcast/releases/episode-001/private-feed.xml').read_text()
assert private == (root / manifest['feed']).read_text().replace('https://agent-logic.ai/podcast/', 'https://agent-logic.ai/_private/podcast/').replace('Prelaunch candidate. Episode content and media may change before launch; do not submit or deploy as a released feed.', 'Private-path playback test only. Not the public launch feed.')
ET.fromstring(private)
print('PASS: private test feed differs only in the URL prefix and test notice')
