# Article Materials — Building a Fully-Local Dictation Stack

**Source:** setup session 2026-08-24, plus the project memory file.
**Note on IPs:** the real LAN address appears throughout my notes below as `192.168.50.163`.
**Genericize it before publishing** — use `<server-lan-ip>` or `192.168.1.x`. Same for the
Moonglade address `192.168.50.169`. Your own rule: never publish internal IPs.

---

## The problem you were solving

You were a heavy user of a **cloud dictation service** (Wispr Flow / Superwhisper class) and
liked its AI cleanup and templates. Two objections: it was cloud-based, and the free tier was
running out. The goal was to keep the front-end experience — hotkey, templates, dictionary —
while moving the AI cleanup onto your own hardware.

The eventual result: **audio never leaves Moonglade, text never leaves your LAN.**

---

## The architecture

```
Moonglade (MacBook Pro M1)
  └─ VoiceInk
       ├─ STT: Parakeet V3 (NVIDIA), on-device
       └─ Cleanup request over LAN
            ↓
Stormwind (Mac mini M4 Pro, 48 GB)
  └─ Ollama (LaunchAgent, under the `zelda` account)
       └─ gemma4:12b-mlx  ·  ~7.7 GB on disk, 8.2 GB resident, 100% GPU
            ↓
       cleaned text lands at your cursor
```

**Measured end-to-end cleanup latency in production: 349 ms and 712 ms** (two real dictations
observed in the Ollama log). Benchmarks during setup ran 450–920 ms.

---

## Decision 1 — Why *not* a frontier model

This is the most counter-intuitive part of the project and probably your best section.

**The framing correction:** optimizing for "frontier" is solving the wrong problem. Dictation
cleanup is a **constrained rewriting task** — fix punctuation, drop fillers, don't touch
meaning. The hard part isn't intelligence, it's **restraint and consistency**.

**A big reasoning model is actively worse at this.** Demonstrated live during setup:
Qwen3.6-35B-A3B burned **299 tokens** narrating its own process ("Identify Filler Words") and
returned an **empty string**. That's the failure mode you're designing against, not an edge case.

The requirements that fell out of this, and why each one:

| Requirement | Why |
|---|---|
| **Non-reasoning** | Thinking tokens are pure latency tax on a task with no reasoning content |
| **Dense, not MoE** | MoE routing introduces variance; you want the *same* edit every time |
| **Mid-size (~12B)** | Capability beyond instruction-following is capability you never use |
| **MLX quant** | Apple Silicon native; keeps it on GPU |

## Decision 2 — Why Gemma 4 12B specifically

**Chosen:** `gemma4:12b-mlx` — **7.7 GB** download, 8.2 GB resident, 256K context.

Reasons, in order of weight:

1. **Dense** — no MoE routing variance, so cleanup behavior is consistent utterance to utterance
2. **Gemma's documented trait** — Google's Gemma line is specifically noted for consistently
   formatted output that obeys instructions, which is exactly this job
3. **Headroom** — 7.7 GB on a 48 GB machine leaves enormous room, so it stays resident and you
   never pay cold-load latency per utterance
4. **256K context** — far more than needed, but free

**Fallback considered and rejected as default:** `qwen3.6:27b-mlx` (20 GB). Genuinely dense and
defaults to non-thinking, but 27B params buy capability you won't use for comma placement.
Still worth an A/B if Gemma ever over-edits.

**Rejected outright:** Qwen3.6-35B-A3B (the MoE already running on Stormwind) — reasoning model,
demonstrated the empty-output failure above.

**A note on how the choice was made:** the first instinct was to use what was already loaded on
Stormwind. You pushed back — "don't be confined by which models are currently available, search
for what's actually suitable, maybe not a reasoning model, maybe not MoE." That reframing is
what produced the right answer. Worth including; it's the actual decision-making moment.

---

## Decision 3 — Why VoiceInk over Superwhisper / Wispr Flow / MacWhisper

**You're right that the base URL was the deciding factor.** The evidence came from reading
VoiceInk's source rather than its marketing page — `OllamaService.swift`:

```swift
static let defaultBaseURL = "http://localhost:11434"
@Published var baseURL: String {          // user-editable, persisted
    UserDefaults.standard.set(baseURL, forKey: "ollamaBaseURL")
}
```

The base URL is a **first-class editable setting**, not hardcoded localhost. It also calls
`fetchModels(baseURL:)`, so VoiceInk queries Ollama's model list and renders **a dropdown of
every model on Stormwind** — model swapping from a menu instead of editing config.

**Why Superwhisper failed the test:** its docs index has **no Ollama or local-LLM page at all**;
custom model providers are an **Enterprise** feature. Its local-first reputation is about
*transcription* — the Whisper half runs on-device, but **the AI cleanup half is cloud.** That's
precisely the layer you were trying to bring home. Paying $8.49/mo or $249 lifetime to keep the
part you want to eliminate is the wrong trade.

Requirements checked against source:

| Requirement | VoiceInk |
|---|---|
| Remote LLM host | `ollamaBaseURL` user setting ✅ |
| Swap local models | `OllamaService.fetchModels(baseURL:)` → live dropdown ✅ |
| Templates like your old modes | `Modes/` + per-mode prompts ✅ |
| Custom dictation dictionary | `CustomVocabularyService.swift` + `DictionaryService.swift` ✅ |

**Licensing / cost angle** (optional, but a good sovereignty sidebar): VoiceInk is **GPLv3,
$29–49 one-time, 7-day trial**. The trial gate is **UI-only** — `trialExpired` appears in three
view files (`LicenseViewModel`, `LicenseManagementView`, `DashboardContent`) and **never** in
`Recorder.swift`, `WhisperState.swift`, or `AIEnhancementService.swift`. `resetTrial()` exists
in the open source. So enforcement is an honor system by deliberate design, and `make local`
builds every feature free. **You chose to pay rather than maintain a self-built binary** —
which is a defensible sovereignty position worth stating: open source is about *licensing*,
not price, and paying for signing/notarization/updates is buying packaging, not features.

---

## The technical traps — the section you asked for

### Trap 1 — The reasoning-model latency tax (the headline finding)

Gemma 4 12B **is** a reasoning model, contrary to the initial assumption. Default behavior cost
**7.6 seconds per cleanup** with hidden thinking tokens.

| Configuration | Latency | Output |
|---|---|---|
| Default (thinking on) | **7.6 s** | correct, but 366 tokens |
| `think:false` on `/api/chat` | 0.43 s | correct — **but VoiceInk can't use this endpoint** |
| **`reasoning_effort:"none"` on `/v1`** | **0.51 s** | correct, 19 tokens |

**A ~15× improvement from one parameter.**

**The trap beneath the trap:** `chat_template_kwargs.enable_thinking` and `options.think` are
**silently ignored** on the `/v1` OpenAI-compatible endpoint. They return correct content while
still burning hidden reasoning tokens — so they look like success. Only `reasoning_effort` works
on `/v1`; `think:false` works only on native `/api/chat`.

This one generalizes well beyond dictation: *any* local reasoning model behind an
OpenAI-compatible endpoint has this failure mode.

### Trap 2 — LaunchAgent under the wrong user

`gui/501` (the `zire` account) **fails to bootstrap** — zire has no console session. Only
`zelda` (uid 502) holds the GUI session, so Ollama must run there:
`launchctl kickstart -k gui/502/com.ollama.server`.

### Trap 3 — Stale Homebrew Ollama

Homebrew shipped **0.15.2**; Gemma 4 needs **≥0.32.x** or the pull fails with an opaque **412**.

### Trap 4 — Long model pulls die with the SSH session

Run them as a **launchd job, not `nohup`**.

### Trap 5 — VoiceInk rejects remote plain HTTP in the obvious place

"Custom Enhancement Models" refuses it: *"Base URL must use HTTPS; plain HTTP is allowed only
for localhost."* **Workaround:** use **AI Models → Local tab → Local Enhancement Services →
Ollama** instead — that panel has no HTTPS check. Enter `host:port` only, no `/v1/...` path.

### Trap 6 — The silent no-op mode

A mode **silently skips enhancement** if it has no prompt **and** no bound keyboard shortcut.
Symptom: transcription works fine, but **no POST ever reaches Ollama**. Both must be set, then
Save Changes. Binding the shortcut was the actual fix that made the pipeline live.

### Trap 7 — Model loading failures

Gemma initially **failed to load** with a vision-tower weights error (the QAT variant was broken
or needed a different loader). Worth a line — "the first thing you try may just be a bad build."

### Trap 8 — DHCP lease

The endpoint depends on a stable IP. **Reserve the lease on the router** — otherwise a lease
change silently breaks dictation months later when you've forgotten why. You did this.

---

## Configuration that matters

| Setting | Value | Why |
|---|---|---|
| `OLLAMA_HOST` | `<server-lan-ip>:11434` | LAN interface only — **not** `0.0.0.0` |
| `OLLAMA_KEEP_ALIVE` | `24h` | Model stays warm on GPU; no cold start per utterance |
| LaunchAgent | `com.ollama.server`, `gui/502` | Survives reboot |
| Logs | `~zelda/Library/Logs/ollama.log` | Verify traffic: grep POSTs from Moonglade's IP |

**Why LAN IP over Tailscale MagicDNS:** deliberate choice. Home-only use case, and **Astrill VPN
breaks MagicDNS** (rewrites `/etc/resolv.conf`, so `*.ts.net` returns NXDOMAIN).

**Verification signals from the Ollama log** — useful concrete detail:
- `100% GPU` — fully on the M4 Pro's GPU, not spilling to CPU
- `UNTIL 24 hours from now` — keep-alive working
- `8.2 GB, 32k context` — comfortable on 48 GB
- POSTs from Moonglade's IP confirm the wire path

---

## Loose ends / honest caveats worth including

- **Quality over time is the real test.** The plumbing worked immediately; whether a 12B model
  has the *restraint* to clean without rewriting only shows across dozens of real dictations.
  If it starts rephrasing, that's prompt tightening or a model swap — both cheap now.
- **Non-fatal audio errors** in the log: `HALC_ProxyIOContext Start failed - error 35`. It
  recovered via batch fallback. If you ever get truncated dictations, that's the cause — likely
  a Bluetooth or input-device conflict.
- **Real-time mode is on** and working. If you see partial or oddly-fragmented cleanup, turning
  it off gives the LLM complete utterances instead of streaming chunks.
- **Settings export** to pretty-printed JSON (`VoiceInk_Settings_Backup.json`) is portable and
  git-friendly — but don't commit it if API keys are ever added.

---

## Suggested framing for the article

The strongest through-line: **"local" is not one decision, it's three** — where the audio is
transcribed, where the cleanup model runs, and whether the app *lets* you point it elsewhere.
Superwhisper is "local" on the first and fails the third. The base URL field is the whole ball
game.

Second-strongest: **the counter-intuitive model choice.** Everyone reaches for the biggest model
they can run. For constrained rewriting, the big reasoning model is measurably worse — 7.6 s and
sometimes an empty string, versus 0.5 s and 19 tokens. That's a concrete, quantified argument
against the instinct, and it's the kind of thing readers remember.

The sovereignty payoff, stated plainly: a cloud free tier ran out, and instead of paying a
subscription you now own the whole pipeline outright — with better latency than the thing it
replaced.
