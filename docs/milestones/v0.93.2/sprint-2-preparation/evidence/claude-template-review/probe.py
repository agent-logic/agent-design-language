import importlib.util,json,os,re,time
from pathlib import Path
from playwright.sync_api import sync_playwright
root=Path('/private/tmp/worker30-new130-baseline/source-snapshot')
out=Path('/private/tmp/worker30-new130-slot-probes'); out.mkdir(exist_ok=True)
os.environ['CHROME_PATH']='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome'
os.environ.pop('CF_CHROME_NO_SANDBOX',None)
spec=importlib.util.spec_from_file_location('pdf',root/'pdf-pipeline/build_pdf.py'); pdf=importlib.util.module_from_spec(spec); spec.loader.exec_module(pdf)
results={}
with sync_playwright() as p:
 b=p.chromium.launch(**pdf.launch_options())
 try:
  c=b.new_context(service_workers='block',accept_downloads=False)
  page=c.new_page(); attempts=[]
  page.route('**/*',lambda r:(attempts.append(r.request.url),r.abort()))
  # Benign local SVG marker and intercepted reserved-domain image; no actual remote request.
  page.set_content('<html><body><svg xmlns="http://www.w3.org/2000/svg" width="200" height="80"><script>window.cfProbeExecuted=true</script><image href="https://probe.invalid/image.png" width="10" height="10"/><text x="5" y="30">Benign isolation probe</text></svg></body></html>')
  results['mermaid_set_content']={'inline_script_executed':page.evaluate('window.cfProbeExecuted===true'),'blocked_by_test_harness_requests':attempts,'boundary':'same set_content insertion as snapshot finish.py; extra test-harness route abort and PDF dead proxy prevent actual egress; not a claim of successful network exfiltration'}
  c.close()
  original=pdf.build_html
  class ProbePage:
   def __init__(self,page): self.page=page
   def __getattr__(self,name):
    if name=='close':
     def close():
      results['pdf_isolation']={'inline_script_executed':self.page.evaluate('window.cfProbeExecuted===true'),'script_tag_present':self.page.locator('script').count()}
      self.page.close()
     return close
    return getattr(self.page,name)
  class ProbeContext:
   def __init__(self,c): self.c=c
   def new_page(self): return ProbePage(self.c.new_page())
  def injected(name,compact=False): return original(name,compact).replace('</body>','<script>window.cfProbeExecuted=true</script><img src="https://probe.invalid/image.png"></body>')
  pdf.build_html=injected
  c=b.new_context(service_workers='block',accept_downloads=False)
  try: pdf.render(ProbeContext(c),'Main',False)
  finally: c.close()
  def variable(name,compact=False):
   rows=''.join(f'<tr><td>VARIABLE_ROW_{i:03}</td><td>'+('Synthetic evidence content ' * (1+i%7))+'</td></tr>' for i in range(100))
   return original(name,compact).replace('</tbody>',rows+'</tbody>',1).replace('The service is well structured and its core ledger logic is sound.',('LONGTOKEN_'*300)+' VARIABLE_CONTENT_END',1)
  pdf.build_html=variable
  c=b.new_context(service_workers='block',accept_downloads=False)
  try:
   start=time.perf_counter(); payload=pdf.render(c,'Main',False); (out/'variable-report.pdf').write_bytes(payload)
   results['variable_report']={'render_ms':(time.perf_counter()-start)*1000,'pages':pdf.page_count(payload),'bytes':len(payload),'boundary':'100 synthetic rows in existing table and long token in existing summary paragraph; isolated sample mutation, not production field renderer'}
  finally: c.close()
 finally: b.close()
(out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
print(json.dumps(results,indent=2))
