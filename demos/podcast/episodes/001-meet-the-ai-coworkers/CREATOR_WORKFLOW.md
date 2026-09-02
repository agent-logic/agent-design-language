# The Cognitive Stack Episode Workflow

This is the working handoff for Darlicia and Codex. Darlicia does not need to
run production commands, manage provider credentials, edit metadata files, or
touch RSS by hand.

## What Darlicia Provides

1. Episode title or question
2. Three to five points the episode should cover
3. Anything that must not be said
4. Preferred tone and approximate length
5. Final approval after reading and listening

## The Four Approvals

1. **Brief approval:** agree on the listener outcome, boundaries, and format.
2. **Script approval:** read the complete script before any final rendering.
3. **Audio approval:** listen to the mastered candidate for content, voices,
   pronunciation, pacing, edits, and unwanted artifacts.
4. **Publish approval:** verify title, summary, artwork, explicit-content truth,
   publication date, and links before the feed changes publicly.

Each approval is independent. Approval at one stage does not authorize the
next stage or publication.

## What Codex Produces

1. A script candidate
2. A clearly labeled voice audition when casting changes
3. A mastered WAV archive and MP3 distribution candidate
4. A transcript reconciled to the final spoken script
5. Show notes, chapter marks, artwork, and episode metadata
6. Audio measurements, provider disclosures, and file digests
7. A private preview for editorial listening

## Review Loop

1. Darlicia supplies the brief and approves it.
2. Codex drafts the complete script. Darlicia edits or approves it.
3. Codex renders the audio and prepares the complete episode package.
4. Darlicia listens for content, pronunciation, pacing, voice separation, and
   tone, then approves or requests specific corrections.
5. Codex makes requested corrections and reruns focused media/feed validation.
6. Daniel or Darlicia explicitly approves publication after reviewing the
   final metadata and preview.

Nothing is added to the public feed or submitted to a directory without the
final approval in step 6.

## Codex Production Route

Codex uses the approved episode script as the rendering source. The renderer
extracts every `### ChatGPT`, `### Gemini`, and `### Claude` turn in order,
records each turn's script provenance, and produces the combined WAV and audio
manifest. Provider credentials remain outside the repository.

```bash
export ADL_PODCAST_OUTPUT_DIR="${ADL_PODCAST_OUTPUT_DIR:-/Volumes/FastWork/the-cognitive-stack-production}"
ADL_PODCAST_AUDIO_SOURCE_DIR="$PWD/demos/podcast/episodes/001-meet-the-ai-coworkers" \
ADL_PODCAST_GEMINI_AUDIO_PROVIDER=openai \
bash adl/tools/demo_v0911_multiagent_podcast_audio.sh "$ADL_PODCAST_OUTPUT_DIR"
```

For a future episode, Codex changes the source directory and output directory,
then performs the same focused listen, loudness, metadata, artwork, feed, and
digest checks before asking for audio approval. Darlicia still provides only
the editorial inputs and approvals above.

## Prompt To Start A Future Episode

> Create an episode of The Cognitive Stack about [topic]. The listener should leave
> understanding [outcome]. Cover [points]. Avoid [constraints]. Aim for [length]
> minutes with [hosts or guests] in a [tone] conversation. Prepare the complete
> brief and script for my approval before generating final audio or changing
> the feed. Do not publish anything without separate publish approval.
