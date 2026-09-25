// Display elapsed playback only; downloaded/buffered bytes never fill the seek bar.
class PodcastPlayer extends HTMLElement {
  static get observedAttributes() { return ['src']; }
  constructor() {
    super();
    const root = this.attachShadow({mode: 'open'});
    root.innerHTML = `
      <style>
        :host{display:block;font:14px system-ui,sans-serif;color:#202124}
        .controls{display:flex;align-items:center;gap:12px;background:#f1f3f4;border-radius:28px;padding:10px 14px}
        button{border:0;background:transparent;color:inherit;font:inherit;cursor:pointer;padding:4px;min-width:36px}
        input{min-width:40px;flex:1;width:100%;accent-color:#2563eb}
        time{font-variant-numeric:tabular-nums;white-space:nowrap}
        .error{margin:8px 0;color:#a21d24}
        @media(max-width:400px){.controls{gap:6px;padding:8px}time{font-size:12px}}
      </style>
      <audio preload="metadata"></audio>
      <div class="controls">
        <button class="play" type="button" aria-label="Play episode">▶</button>
        <time>0:00 / —</time>
        <input type="range" min="0" max="100" value="0" step="0.1" disabled aria-label="Playback position">
        <button class="mute" type="button" aria-label="Mute audio">♪</button>
      </div>
      <p class="error" role="status" hidden></p>`;
    this.audio = root.querySelector('audio');
    const play = root.querySelector('.play'), mute = root.querySelector('.mute');
    const seek = root.querySelector('input'), time = root.querySelector('time'), error = root.querySelector('.error');
    const format = seconds => `${Math.floor(seconds / 60)}:${String(Math.floor(seconds % 60)).padStart(2, '0')}`;
    const update = () => {
      const duration = this.audio.duration;
      const ready = Number.isFinite(duration) && duration > 0;
      const current = this.audio.currentTime || 0;
      seek.disabled = !ready;
      seek.max = ready ? duration : 100;
      seek.value = current;
      seek.setAttribute('aria-valuetext', `${format(current)} of ${ready ? format(duration) : 'unknown duration'}`);
      time.textContent = `${format(current)} / ${ready ? format(duration) : '—'}`;
      play.textContent = this.audio.paused ? '▶' : '❚❚';
      play.setAttribute('aria-label', this.audio.paused ? 'Play episode' : 'Pause episode');
      mute.textContent = this.audio.muted ? '×' : '♪';
      mute.setAttribute('aria-label', this.audio.muted ? 'Unmute audio' : 'Mute audio');
      mute.setAttribute('aria-pressed', String(this.audio.muted));
    };
    play.addEventListener('click', async () => {
      if (!this.audio.paused) { this.audio.pause(); return; }
      try { await this.audio.play(); error.hidden = true; }
      catch { error.textContent = 'Playback could not start. Please try again.'; error.hidden = false; }
    });
    mute.addEventListener('click', () => { this.audio.muted = !this.audio.muted; });
    seek.addEventListener('input', () => { if (!seek.disabled) this.audio.currentTime = Number(seek.value); update(); });
    for (const event of ['loadedmetadata', 'durationchange', 'timeupdate', 'play', 'pause', 'ended', 'volumechange', 'emptied']) this.audio.addEventListener(event, update);
    this.audio.addEventListener('error', () => { error.textContent = 'Audio could not load. Please refresh and try again.'; error.hidden = false; });
    update();
  }
  attributeChangedCallback(name, oldValue, value) {
    if (name === 'src' && value && value !== oldValue) this.audio.src = value;
  }
  disconnectedCallback() { this.audio.pause(); }
}
customElements.define('podcast-player', PodcastPlayer);
