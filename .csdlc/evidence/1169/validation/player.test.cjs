const fs = require('node:fs');
const vm = require('node:vm');
const assert = require('node:assert/strict');
const path = require('node:path');
const root = path.resolve(__dirname, '../../../..');
class Element {
  constructor() { this.events = {}; this.attrs = {}; this.paused = true; this.currentTime = 0; this.duration = NaN; this.muted = false; this.style = {setProperty: (k,v) => {this.attrs[k] = v;}}; }
  addEventListener(k,f) { this.events[k] = f; }
  setAttribute(k,v) { this.attrs[k] = v; }
  pause() { this.paused = true; this.events.pause?.(); }
  async play() { this.paused = false; this.events.play?.(); }
}
let Player;
const nodes = Object.fromEntries(['audio','.play','.mute','input','time','.error'].map(k => [k,new Element()]));
class Host {
  attachShadow() { return {set innerHTML(v) {}, querySelector:k => nodes[k]}; }
  getAttribute(k) { return k === 'duration' ? '550.056' : null; }
}
vm.runInNewContext(fs.readFileSync(path.join(root,'demos/podcast/studio/podcast-player.js'),'utf8'), {HTMLElement:Host,customElements:{define:(n,c) => { Player=c; }}});
(async () => {
  const player = new Player(), audio = nodes.audio, seek = nodes.input;
  assert.equal(nodes.time.textContent,'0:00 / 9:10');
  assert.equal(seek.attrs['--played'],'0%');
  audio.duration = 550.056;
  audio.buffered = {length:1,end:() => 200};
  audio.events.loadedmetadata();
  assert.equal(seek.attrs['--played'],'0%');
  audio.currentTime = 275.028; audio.events.timeupdate();
  assert.equal(seek.attrs['--played'],'50%');
  seek.value = '60'; seek.events.input(); assert.equal(audio.currentTime,60);
  await nodes['.play'].events.click(); assert.equal(nodes['.play'].attrs['aria-label'],'Pause episode');
  await nodes['.play'].events.click(); assert.equal(nodes['.play'].attrs['aria-label'],'Play episode');
  nodes['.mute'].events.click(); audio.events.volumechange();
  assert.equal(nodes['.mute'].attrs['aria-label'],'Unmute audio');
  assert(nodes['.mute'].innerHTML.includes('<svg'));
  player.attributeChangedCallback('src',null,'audio/meet-the-ai-coworkers.mp3');
  assert.equal(audio.src,'audio/meet-the-ai-coworkers.mp3');
  audio.currentTime=0; audio.events.emptied(); assert.equal(seek.attrs['--played'],'0%');
  console.log('PASS: zero buffered progress, elapsed fill, seek, play/pause, mute, source and reset');
})().catch(e => { console.error(e); process.exitCode=1; });
