from html.parser import HTMLParser
from urllib.parse import urljoin
from urllib.request import urlopen
from pathlib import Path
import xml.etree.ElementTree as ET
class Links(HTMLParser):
 def __init__(self):super().__init__();self.hrefs=[];self.ids=[];self.assets=[]
 def handle_starttag(self,tag,attrs):
  a=dict(attrs)
  if 'id' in a:self.ids.append(a['id'])
  if tag=='a':self.hrefs.append(a.get('href'))
  if tag in ('script','img') and 'src' in a:self.assets.append(a['src'])
base='https://agent-logic.ai/podcast/'
root=Path(__file__).resolve().parents[4]
s=(root/'demos/podcast/index.html').read_text();p=Links();p.feed(s)
for href in p.hrefs:
 if href=='{{ ep.href }}':
  assert 'href: ep.num === 1 ? "#listen" : null' in s
  print('PASS Episode 1 -> #listen; proposed episodes have no destination');continue
 assert href
 if href.startswith('#'):
  assert p.ids.count(href[1:])==1;print('PASS anchor',href)
 elif href.startswith('mailto:'):
  assert href=='mailto:podcast@agent-logic.ai';print('PASS email target (delivery not tested)',href)
 else:
  with urlopen(urljoin(base,href),timeout=30) as r:
   data=r.read();assert r.status==200
   if href=='feed.xml':assert ET.fromstring(data).findtext('./channel/title')=='The Cognitive Stack'
   else:assert b'Agent Logic' in data
  print('PASS destination',href)
for src in p.assets:
 with urlopen(urljoin(base,src),timeout=30) as r:assert r.status==200
 print('PASS asset',src)
print('PASS',len(p.hrefs),'anchor elements; outlet placeholders excluded; browser click behavior not exercised')
