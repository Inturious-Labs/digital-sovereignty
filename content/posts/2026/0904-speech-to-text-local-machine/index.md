---
title: "Use Local LLM to Enable AI Enhancement for Speech-to-Text Dictation"
date: 2026-09-04T22:00:00+08:00
slug: use-local-llm-to-enable-ai-enhancement-for-speech-to-text-dictation
draft: false
description: "how to set up speech-to-text dictation process on local device to preserve privacy"
categories:
  - "Digital Life"
series:
  - "One-Person Company"
images: ["mic.jpg"]
keywords: ["decentralization", "digital sovereignty", "dictation", "speech-to-text", "voiceink", "gemma", "LLM", "AI"]
enable_rapport: true
---

![Mic](mic.jpg)

## Problem

Speech-to-text dictation is becoming a way of life for people with heavy AI usage - coding or researching. It’s much faster to speak than to type. There are two layers to the underlying technology: **transcription** and **enhancement**. 

Transcription is largely a solved problem in 2026 - chips on local devices like MacBook Pro and iPhone can do a very decent job of faithfully capturing the raw audio and converting that into text with high accuracy. 

While the transcription has always been solid, the AI enhancement turned out to be a killer feature with the rise of vibe coding in early 2025. An AI LLM can:

- remove filler words (“en”, “oh”, “ah”)
- remove weasel words (“basically”, “I feel like”, “to be fair”, etc)
- correct contextual mistakes
- transform rambling monologue into well-structured action items
- normalize inconsistent formats

I’ve been a fan of [Typeless](https://www.typeless.com) for a few months. It’s become an indispensable part of my coding sessions. 

However, as my AI usage picked up, three problems started surfacing:

1. Network traffic congestion. Sometimes when I need it the most, it’s not available because the server is overloaded with too many concurrent users.
2. It’s a cloud-based application, so my private conversation not only overflows into Claude Code, which cannot be avoided so far, but also has a copy sitting on the servers of Typeless. That’s a show-stopper for me.
3. Though it offers a generous free tier, the paid version is what the company drives its users toward. I don’t want to be bound to another paid SaaS. 


I started exploring if it’s possible to achieve speech-to-text dictation with both transcription and enhancement entirely on a local device, so that I can preserve my privacy AND enjoy this productivity boost.

## Solution

The end-to-end dictation workflow consists of several pieces:

- a dictation client that runs on my MacBook Pro and is open-source.
- a machine that can serve a local LLM to perform enhancement
- a robust local LLM that can effectively handle enhancement with low latency

### Dictation Client

I chose [VoiceInk](https://tryvoiceink.com). It’s open-source but with buy-out options to spare me the troubles of keeping the software up to date. 

The single most important feature I need, which makes VoiceInk stand out, is that I need to be able to specify **base URL** in this client. It cannot hardcode this URL because I would need to call a local LLM with `localhost:xxxx`.  

Another popular dictation client, **Superwhisper**, does not allow me to specify **Ollama** or a local LLM page at all. Custom model provider is an **Enterprise** feature. Its local-first reputation is about transcription, but the AI cleanup half is on the cloud.

### LLM for Enhancement

The more exciting part is to choose the right local LLM to handle this task. The quest is not to pursue the single most advanced frontier model of the month. Dictation cleanup is a **constrained rewriting** task, not a reasoning one. There is no problem to solve, no ambiguity to resolve, nothing to plan. The entire job is restraint - clean up the language, preserve the substance, do it the same way every time.

Frontier models are optimized for the opposite. They’re rewarded for elaborating, for being helpful beyond the literal instruction. Pointing that at a transcript and it doesn’t clean the sentence, it improves it. I don’t want that. 

So it shall meet several criteria:

- **Non-reasoning**. Thinking tokens are pure latency tax that is not necessary.
- **Dense**, not MoE (Mixture of Experts). MoE routing introduces variance. I want the same edit every time.
- **Mid-size** (~12B). It needs to stay resident in the memory.
- **MLX** quant. Apple Silicon native framework makes better use of its GPU than the more universal **GGUF**.

After some experiments, I went with **Google**’s `gemma:12b-mlx` that is `7.7GB` in size, `12.4B` parameters, `48` layers, and no expert routing. It’s genuinely dense, so the same input produce the same edit. Gemma’s family trait is obedience - well-formatted output that follows the instruction rather than improvising it. 7.7GB on my `48GB M4 MacMini` leaves enormous headroom, so it stays resident, and I never need to pay cold-load latency per utterance.

### Hosting Local LLM

The dictation client VoiceInk runs on my daily driver, an M1 Pro MacBook Pro, but the enhancement model runs on a Mac mini (M4 Pro, 48GB) on the same LAN, reachable at `<server-lan-ip>:xxxxx`.

Splitting the two machines is deliberate. A 12B model holds ~16GB resident and pins the GPU for every request. On a laptop, that means fan noise, battery drain, and contention with the editor I’m working on. On an always-on desktop, it means a model that is already warm when I press the hotkey.

There are two ways to deploy a local AI LLM - **LM Studio** and **Ollama**. For my **Hermes** agent that runs on the same Mac mini, I use AI models served by LM Studio. But for this dictation workflow, I go with Ollama. I don’t need the GUI from LM Studio. I want to be able to configure the model with just SSH.

Also, make sure the Ollama service is run as a **LaunchDaemon**, not a LaunchAgent.

macOS has two ways to keep a background process alive. A LaunchAgent runs on behalf of a logged-in user, requiring a GUI session. A LaunchDaemon runs on behalf of the system. It’s owned by `root`, and starts at boot whether or not anyone has ever logged in. That’s the right choice for such a server. 

## Result

What I get out of this setup is 100% on parity with what **Typeless** gives me, minus the worry about privacy and anxiety from network congestion. The AI enhancement by the Gemma model served via Ollama on my Mac mini responds to my every utterance swiftly and reliably, every time.

This has changed my coding rhythm considerably. In the past, when I still needed to do a lot of typing, I would just focus on one task/panel of Claude Code at a time, because switching context and giving instructions through typing is just a lot of work. I’ve switched to 100% dictation in the last few days and found myself being able to juggle 3-4 Claude Code panels easily. After all, speaking is 5x-10x faster than typing.

This has unlocked a significant productivity boost. I upgraded my **Claude Code** account to the $100/month Max tier and bought a pair of **DJI Mic Mini 2** microphones, which is much better for vibe coding than **AirPods Pro** (vibe coding only needs to output audio, not to take in audio). 

Kicking into high gear in 2026!