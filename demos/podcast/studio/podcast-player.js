// Display elapsed playback only; downloaded/buffered bytes never fill the seek bar.
class PodcastPlayer extends HTMLElement {
  static get observedAttributes() { return ['src']; }
  constructor() {
    super();
    const root = this.attachShadow({mode: 'open'});
    const icon = paths => `<svg viewBox="0 0 24 24" width="22" height="22" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">${paths}</svg>`;
    const icons = {
      play: icon('<path d="M8 5l11 7-11 7z" fill="currentColor" stroke="none"/>'),
      pause: icon('<path d="M8 5v14M16 5v14" stroke-width="4"/>'),
      sound: icon('<path d="M11 5L6 9H3v6h3l5 4z"/><path d="M15 8a6 6 0 010 8M18 5a10 10 0 010 14"/>'),
      muted: icon('<path d="M11 5L6 9H3v6h3l5 4z"/><path d="M16 9l5 6M21 9l-5 6"/>')
    };
    root.innerHTML = `
      <style>
        :host{display:block;font:14px system-ui,sans-serif;color:#202124}
        .controls{display:flex;align-items:center;gap:12px;background:#f1f3f4;border-radius:28px;padding:10px 14px}
        button{display:grid;place-items:center;border:0;background:transparent;color:inherit;font:inherit;cursor:pointer;padding:8px;width:40px;height:40px;flex-shrink:0;border-radius:50%}
        button:hover{background:#e2e8f0}button:focus-visible,input:focus-visible{outline:2px solid #2563eb;outline-offset:3px}
        .play{background:#2563eb;color:white}.play:hover{background:#1d4ed8}
        input{appearance:none;-webkit-appearance:none;min-width:40px;flex:1;width:100%;height:6px;margin:16px 0;border-radius:3px;background:linear-gradient(to right,#2563eb var(--played,0%),#cbd5e1 var(--played,0%));cursor:pointer}
        input::-webkit-slider-thumb{-webkit-appearance:none;width:12px;height:12px;border:0;border-radius:50%;background:#2563eb}
        input::-moz-range-thumb{width:12px;height:12px;border:0;border-radius:50%;background:#2563eb}
        input:disabled{cursor:default;opacity:.6}
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
      seek.style.setProperty("--played", `${ready ? Math.max(0, Math.min(100, current / duration * 100)) : 0}%`);
      seek.setAttribute('aria-valuetext', `${format(current)} of ${ready ? format(duration) : 'unknown duration'}`);
      const knownDuration = ready ? duration : Number(this.getAttribute('duration'));
      time.textContent = `${format(current)} / ${knownDuration > 0 ? format(knownDuration) : '—'}`;
      play.innerHTML = this.audio.paused ? icons.play : icons.pause;
      play.setAttribute('aria-label', this.audio.paused ? 'Play episode' : 'Pause episode');
      mute.innerHTML = this.audio.muted ? icons.muted : icons.sound;
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
