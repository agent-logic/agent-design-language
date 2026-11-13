from pathlib import Path
import zipfile,base64,html,json,hashlib,argparse
args=argparse.ArgumentParser(description='Render GCP diagram from official downloaded icon archives')
args.add_argument('--core-icons',required=True,type=Path)
args.add_argument('--legacy-icons',required=True,type=Path)
a=args.parse_args()
p=Path(__file__).resolve().parent
z=zipfile.ZipFile(a.legacy_icons);core=zipfile.ZipFile(a.core_icons);icons={};records={}
for key in ['cloud_load_balancing','cloud_nat','cloud_router','firestore','pubsub','secret_manager','cloud_dns','cloud_build','cloud_logging','cloud_cdn','cloud_run','cloud_storage','compute_engine']:
 archive=z;n=key+'/'+key+'.svg'
 if key in ['compute_engine','cloud_storage','cloud_run']:
  term={'compute_engine':'Compute Engine','cloud_storage':'Cloud Storage','cloud_run':'Cloud Run'}[key];n=next(x for x in core.namelist() if '/'+term+'/SVG/' in x and x.endswith('.svg'));archive=core
 b=archive.read(n);icons[key]='data:image/svg+xml;base64,'+base64.b64encode(b).decode();records[key]={'member':n,'sha256':hashlib.sha256(b).hexdigest(),'collection':'core' if archive==core else 'official legacy'}
s=['<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="1800" height="1320" viewBox="0 0 1800 1320"><rect width="1800" height="1320" fill="white"/><defs><marker id="a" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse"><path d="M0 0L10 5L0 10Z" fill="#5f6368"/></marker></defs>']
def text(x,y,t,size=16):s.append(f'<text x="{x}" y="{y}" font-family="Arial" font-size="{size}" fill="#202124">{html.escape(t)}</text>')
def box(x,y,w,h,label,fill='#f8faff',dash=False):s.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" fill="{fill}" stroke="#4285f4"'+(' stroke-dasharray="6 4"' if dash else '')+'/>');text(x+12,y+25,label)
def node(x,y,w,key,label,sub):box(x,y,w,66,'','white');s.append(f'<image x="{x+8}" y="{y+10}" width="40" height="40" xlink:href="{icons[key]}"/>');text(x+56,y+26,label,15);text(x+56,y+49,sub,12)
def line(x,y,xx,yy):s.append(f'<path d="M{x} {y}L{xx} {yy}" stroke="#5f6368" fill="none" marker-end="url(#a)"/>')
text(30,40,'CodeFriend | persistent GCP equivalent — PROPOSED',28);text(30,69,'us-east4 proposed • three environments • two selected zones • compute/project placement TBD',17)
node(30,95,550,'cloud_dns','Cloud DNS + managed HTTPS certificates','Three delegated zones; independent environment names')
node(610,95,550,'cloud_build','GitHub + Cloud Build proposal','Isolated trust roles • qualify producer before use')
node(1190,95,580,'cloud_storage','Immutable artifact / proof store','Generation + digest pinned; same bytes promoted')
for x,name in [(30,'DEV'),(620,'STAGING'),(1210,'PRODUCTION')]:
 box(x,200,560,1035,name+' | project choice OPEN');node(x+35,250,490,'cloud_load_balancing','External Application LB + Cloud CDN','Public HTTPS front door; private backend VMs')
 box(x+18,345,524,635,'Global VPC | private regional app/worker subnets')
 node(x+35,395,490,'cloud_nat','Regional Public NAT + Cloud Router','Distributed egress across zones; IP/port sizing gate')
 for xx,zone in [(x+35,'ZONE A'),(x+290,'ZONE B')]:
  box(xx,495,235,360,zone,'white',True)
  node(xx+8,540,219,'compute_engine','Website + gateway','App MIG • capacity TBD')
  node(xx+8,695,219,'compute_engine','Worker capacity','Worker MIG • TBD')
  line(xx+117,540,xx+117,461)
 line(x+280,316,x+280,345);text(x+36,887,'App MIG and worker MIG each select both zones',14);text(x+36,914,'IAP TCP + OS Login; no VM public addresses',14);text(x+36,941,'Google APIs via Private Google Access',14)
 node(x+35,1004,235,'firestore','Firestore candidate','Adapter decision gate')
 node(x+290,1004,235,'pubsub','Pub/Sub + DLQ','Ack/replay mapping')
 node(x+35,1100,235,'secret_manager','Secret Manager','Versioned secrets')
 node(x+290,1100,235,'cloud_logging','Logging/Monitoring','Retention + alerts')
 text(x+35,1203,'Regional services logical; Cloud Run broker placement gated',12)
text(30,1270,'AWS DynamoDB selection remains unchanged. GCP Firestore is a proposal; no multi-active SQLite or implied migration.',15)
text(30,1300,'Official Google Cloud core + legacy icons, unmodified • regional subnets are not AWS zonal subnets • no deployment approval',14)
s.append('</svg>');(p/'GCP_ARCHITECTURE.svg').write_text(''.join(s));(p/'ICON_PROVENANCE.json').write_text(json.dumps({'source':'https://cloud.google.com/icons','archives':{'core':{'url':'https://services.google.com/fh/files/misc/core-products-icons.zip','sha256':hashlib.sha256(a.core_icons.read_bytes()).hexdigest()},'legacy':{'url':'https://services.google.com/fh/files/misc/google-cloud-legacy-icons.zip','sha256':hashlib.sha256(a.legacy_icons.read_bytes()).hexdigest()}},'icons':records,'unmodified':True,'layout':'custom SVG derived from PLAN.md'},indent=2)+'\n')
