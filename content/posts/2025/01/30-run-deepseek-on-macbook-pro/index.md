---
title: "Run DeepSeek on MacBook Pro"
date: 2025-01-30T09:11:53.072000+00:00
slug: run-deepseek-on-macbook-pro
draft: false
description: "Use the most advanced AI LLM on your home computer"
categories:
  - "newsletter"
---

A few days ago Chinese AI startup DeepSeek released its latest R1 large language model (“LLM”) and took the whole world by surprise. It uses only a tiny fraction of the GPU machine cost of its much bigger rivals like chatGPT from OpenAI, but delivers the same or even better result. It’s open source too, so everyone can use its service for free. The world’s most valuable company NVIDIA lost nearly 17% of its market cap overnight, as R1’s superb performance suggested that piling up NVIDIA’s GPU machines to train LLMs with brute force might not the only way to advance AI.

![](././98252351-50dd-4286-9fd5-dce7a9b95633_2082x1118.jpeg))

Yesterday I managed to deploy DeepSeek on my MacBook Pro. Here’s how I did it. It’s actually quite simple. The advantage of running an LLM on your home computer is that you can use it without relying on the servers of DeepSeek, which are currently humming full speed ahead and frequently can’t connect due to high traffic. Another benefit is that the questions you throw to DeepSeek - the prompts, will not be sent to the server and potentially be exposed to others. Your prompts reveal a lot about who you are and what you do. It’s best to keep them private on your own local machine.

Here’s my MacBook Pro:

* Apple M1 Pro, 14-inch, 2021
* 16 GB memory
* macOS: Sequoia 15.2
* Hard Disk: 1 TB SSD

Open up [iTerm2](https://iterm2.com), which is a feature-rich replacement for Mac’s default Terminal.app, and install [Ollama](https://ollama.com/download/mac) in [Homebrew](https://brew.sh), Mac’s package manager.

```None
$ brew install ollama
```

After installation, check the version.

```None
$ ollama --version
ollama version is 0.5.7a
```

Spin up the Ollama server:

```None
$ ollama serve
```

It looks something like this:

[![](./7ba8553c-9f64-42a7-9c9f-999e61346107_1050x243.webp)](./7ba8553c-9f64-42a7-9c9f-999e61346107_1050x243.webp)

You can pick a model that your Mac’s memory and hard disk allow. The bigger the number in front of “B” (for billion of parameters), the more accurate the model is. I’ve downloaded 1.5B, 8B and 14B model.

<https://github.com/deepseek-ai/DeepSeek-R1>

[![](./de36818b-a348-46dc-aa5e-b5a680185c70_815x377.png)](https://github.com/deepseek-ai/DeepSeek-R1?tab=readme-ov-file)

Keep this Terminal (iTerm2) window open, open another Terminal window and download DeepSeek R1’s 1.5B model.

```None
$ ollama run deepseek-r1:1.5b
```

If your network is slow, this downloading process may take a few tries, but don’t worry, the download will resume where you left it off. If you have a VPN service that can route your machine through a US IP address, it will speed up the download quite a bit.

![](././e4f99be6-ffc5-40a4-ac77-45b7988780ae_929x428.png))

Once the download/installation is successful, you will see a prompt. Time to fire away your questions! You can now play around with a local copy of DeepSeek, the most advanced AI that resides on your laptop.

![](././7c0c9f37-f22e-4e9e-9856-5e324541ef4f_712x496.png))

Type /? to see the prompt menu

![](././1de3dc25-b504-4d33-817d-e6ffd83f0bbb_381x701.png))

Two useful prompts:

* /bye to close out this session
* /clear to clear the session context

If you don’t /clear, DeepSeek will treat the dialogue as an ongoing, continuous conversation and sometimes over-think too much.

To restart DeepSeek, just type the same command before, while keeping the Ollama window open.

```None
$ ollama run deepseek-r1:1.5b
```

You can also try out bigger versions of R1.

```None
$ ollama run deepseek-r1:8b
$ ollama run deepseek-r1:14b
```

Experience DeepSeek’s famous “chain-of-thought (COT)” yourself. It’s quite amusing, though the locally run, distilled model is much slower than what you can get straight out of <https://chat.deepseek.com>. This is the price for privacy.

![](././26e60ca9-9353-43c7-a809-ed088c5479e7_1012x805.png))

To run DeepSeek R1’s undistilled model will [require 768 GB memory and quite a few M4 Mac machines](https://x.com/carrigmat/status/1884244369907278106), which is beyond the reach for most consumers. Running the distilled model is a lot easier and could be done by just anyone. Try it out!
