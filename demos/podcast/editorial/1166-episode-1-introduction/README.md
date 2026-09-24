# Episode 1 introduction — script approval packet

Status: the actual-speaker script is approved and replacement audio has been produced. The 9:10 local candidate awaits human listening and audio acceptance. No public release has occurred.

Listen to [the finished MP3](audio-candidate/episode.mp3). [Episode metadata](audio-candidate/episode.json) and [production evidence](audio-candidate/production.json) accompany the WAV archive, transcript, measured chapters and show notes.

Read [the complete current dialogue](own-words/script.md) and view [the supplied cover](cognitive_stack_cover_3000.png). This packet belongs to [issue #1166](https://github.com/agent-logic/agent-design-language/issues/1166). It is a proposal for the existing Episode 1, not a new release.

## Authorship and disclosure

Actual OpenAI `gpt-5.5`, Gemini `gemini-3.1-pro-preview`, and Claude `claude-opus-4-8` responses supply their respective spoken turns. Each received a shared episode brief, a short topical cue, a word target and the preceding conversation. The speakers chose their own wording and examples. This is an editorially guided sequential text exchange, not a live audio conversation or independently chosen episode agenda.

[The generation packet](own-words/manifest.json) retains prompts, outputs, model identifiers, completion metadata and hashes. Accuracy corrections are requested from the original speaker; original turns and revision prompts are retained. Later turns were generated against the original conversation; revised turns preserve those conversational connections. No editor wrote replacement speech. These were direct provider API calls through the repository adapter, not an ADL Runtime execution proof.

The earlier [single-writer draft](script.md), its review and validation remain historical evidence and are superseded as the performance candidate. Their review does not approve this new dialogue. The user approved the complete new dialogue before rendering; the exact approved script hash and relayed approval are retained in the generation manifest.

## Pacing plan

The current dialogue contains **1,440 spoken words** across 18 turns. After the approved conversational-opening revision, the mastered MP3 measures **550.056 seconds (9:10)**, within the 9–11 minute target. The opening and closing were revised at the user’s request, including a DeepSeek special-guest teaser. The other sixteen retained segments were reused. A pitch-preserving tempo factor of 0.978564 was applied during mastering to maintain the requested episode length. Chapter starts are derived from the measured segments and that pacing factor.

## New artwork and preserved candidate

The supplied cover is a 3000 × 3000 RGB PNG. It was copied byte-for-byte, without redesign or conversion. [Artwork provenance](artwork-provenance.json) records its source, size and SHA-256. The supplied PNG is embedded unchanged in the replacement MP3 candidate. It has not been publicly uploaded or substituted into the existing feed.

[Preservation manifest](preserved-candidate.json) records the existing 77 tracked podcast/preview files at the starting revision, including the 18:32 MP3, WAV archive, old artwork, script, transcript, manifests, feed and HTML. Their bytes remain unchanged. Git history and these retained worktree files preserve the old candidate; no immutable S3 evidence was rewritten and no remote archive operation is claimed.

Stable GUID: `agent-logic-the-cognitive-stack-episode-001`.

The podcast landing page and studio HTML, design and copy are outside scope. Do not regenerate or deploy the webpage. Issues #1165/#1167 own the separate RSS correction worktree; coordinate integration after that work settles instead of overwriting it.

## Approval and subsequent delivery

1. Complete: the human editor approved this complete script and its disclosed authorship format. The cover is the supplied replacement candidate; no redesign is proposed.
2. Speech rendering and mastering complete: measured MP3 duration 550.056 seconds (9:10). Full human listening and audio acceptance remain pending.
3. Build a separate replacement episode candidate. Synchronize spoken transcript, show notes, measured chapters, duration, enclosure byte count/hash, selected cover and any embedded artwork derivative. Reconcile feed/media references with the RSS correction work. Do not edit webpage HTML, studio design or copy.
4. Preserve the old candidate until the replacement is accepted. Keep replacement publication date unset and publication status on hold. Script approval is not audio acceptance or public launch approval.
5. Obtain audio acceptance and explicit launch/registration authorization through the responsible owner before any public action.

The requested registration target is **Thursday, September 24, 2026**, with launch targeted for **Friday, September 25**. These are schedule targets, not approval or an episode publication date. Script approval, audio rendering/listening, package verification and registration readiness remain dependencies; directory processing time is not guaranteed. Delayed script approval or a required authorship rewrite should be raised immediately rather than bypassing a gate.

The user authorized PR delivery of this completed audio candidate for external review. Human listening acceptance and public launch are separate. The focused native proof adapter follows the established podcast-feed route: one Cargo test executes the actual Python package-integrity checks. It checks retained media and provenance; it does not claim human listening or runtime behavior proof.
