// Delegate to rendered elements, which are created after the page runtime boots.
document.addEventListener('click', async event => {
  const target = event.target instanceof Element ? event.target : null;
  if (target?.closest('a[data-play-episode]')) {
    // Keep the anchor's normal scrolling, and start playback in this user gesture.
    const player = document.querySelector('#listen podcast-player');
    if (player && typeof player.play === 'function') player.play();
  }
});
