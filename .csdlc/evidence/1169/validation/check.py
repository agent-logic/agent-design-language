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
from email.utils import parsedate_to_datetime
assert item.findtext('pubDate') == manifest['publication_time']
assert parsedate_to_datetime(manifest['publication_time']).tzinfo is not None
assert manifest['status'] in ('authorized_for_publication', 'published')
for asset in manifest['assets']:
    payload = (root / asset['source']).read_bytes()
    approved = (root / asset['approved_source']).read_bytes()
    if asset['source'].endswith('-transcript.md'):
        assert payload.split(b'### ChatGPT', 1)[1] == approved.split(b'### ChatGPT', 1)[1]
    else:
        assert payload == approved
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
page = (root / 'demos/podcast/index.html').read_text()
assert page.count('Sep 24, 2026') == 2 and page.count('length: "9:10"') == 2
assert '18:32' not in page and 'Aug 10' not in page
assert page.count('length: "TBD"') == 9
from zoneinfo import ZoneInfo
assert parsedate_to_datetime(manifest['publication_time']).astimezone(ZoneInfo('America/Los_Angeles')).date().isoformat() == manifest['public_availability_date'] == '2026-09-24'
episode_page = (root / 'demos/podcast/episodes/meet-the-ai-coworkers/index.html').read_text()
assert '2026-09-24T17:40:26-07:00' in episode_page and '9:10' in episode_page
assert not subprocess.check_output(['git','diff','2fbf1237abd4d5933b2dcb7ca60e54626b413420','--','demos/podcast/editorial/1166-episode-1-introduction'], cwd=root)
print('PASS: feed metadata, exact audio/artwork copies and approved transcript turns, hashes/bytes, retained MP3 hash and measured WAV duration, correct page dates/durations and unchanged approved source')

private = (root / 'demos/podcast/releases/episode-001/private-feed.xml').read_text()
private_tree = ET.fromstring(private.replace('https://agent-logic.ai/_private/podcast/', 'https://agent-logic.ai/podcast/'))
public_tree = ET.parse(root / manifest['feed']).getroot()
season = public_tree.find('channel/item/{http://www.itunes.com/dtds/podcast-1.0.dtd}season')
assert season is not None and season.text == str(manifest['season']) == '1'
public_tree.find('channel/item').remove(season)  # Private test snapshot predates season assignment.
for tree in (private_tree, public_tree):
    for parent in tree.iter():
        for child in list(parent):
            if child.tag in ('pubDate', 'lastBuildDate'):
                parent.remove(child)
    for node in tree.iter():
        node.text = (node.text or '').strip()
        node.tail = ''
assert ET.tostring(private_tree) == ET.tostring(public_tree)
print('PASS: approved private feed matches public feed except URL prefix and publication dates')

from html.parser import HTMLParser
class TranscriptText(HTMLParser):
    def __init__(self): super().__init__(); self.parts=[]
    def handle_data(self,data): self.parts.append(data)
parser=TranscriptText(); parser.feed(episode_page.split('<h2>Transcript</h2>',1)[1].split('</main>',1)[0])
import re
spoken=(root / manifest['assets'][1]['source']).read_text().split('### ChatGPT',1)[1]
expected='ChatGPT'+spoken.replace('### ', '')
assert re.sub(r'\s+',' ',''.join(parser.parts)).strip() == re.sub(r'\s+',' ',expected).strip()
print('PASS: episode-page transcript matches approved spoken dialogue')

assert item.findtext("pubDate").endswith(" GMT")
assert channel.findtext("lastBuildDate").endswith(" GMT")
assert parsedate_to_datetime(channel.findtext("lastBuildDate")) >= parsedate_to_datetime(item.findtext("pubDate"))

assert 'href="https://podcasts.apple.com/us/podcast/the-cognitive-stack/id6815902769"' in page
assert 'href="https://open.spotify.com/show/3KnDrv4aPTtjm7vgOzFIyM"' in page
assert 'Amazon Music / Audible — pending</span>' in page
assert 'YouTube Music — pending</span>' in page
