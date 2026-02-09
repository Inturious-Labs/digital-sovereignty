---
title: "Beyond the Hype: 10 Real-World Truths About Living with OpenClaw AI Assistant"
date: 2026-02-10T12:00:00+08:00
slug: beyond-the-hype-10-real-world-truths-about-living-with-openclaw-ai-assistant
draft: false
description: "after using OpenClaw extensively for more than two weeks, here's my honest take on the AI assistant and how it may or may not change our lives."
categories:
  - "AI Coding"
series:
  - "One-Person Company"
images: ["hero.jpg"]
keywords: ["digital sovereignty", "AI", "OpenClaw", "Claude Code", "Anthropic"]
enable_rapport: true
---

![Featured Image](hero.jpg)

The entire tech world has been head over heels over **OpenClaw** in the last two weeks. I drank the Kool-Aid and started playing with Clawdbot on Jan 24. After a test drive on a dormant `2018 MacMini`, right away I ordered a new `M4 Pro MacMini` with 48GB RAM and 1TB hard disk - something I've been procrastinating for more than a year. The first impression of Clawdbot gave me the conviction that this would be the future of AI and probably the Internet as we know it. Within 24 hours that particular model's price surged by RMB 1,000 on JD.com when the buzz of Clawdbot hit the Chinese tech media like a tsunami. Within 2-3 days, MacMinis are flying off the shelf around the globe.

What is OpenClaw, really? 

- For the uninitiated, this is what Apple's **Siri** should have become but failed to land - a true 24/7 stand-by AI assistant. It's **Jarvis** from **Iron Man** and Scarlet Johansson in the 2013 film **Her**. 
- For the more technical, this is essentially LLM + local persistent memory + Instant Messenger tool. 
- For the enthusiasts of "second brain" method enabled by Zettelkasten-style note-taking, this is **Obsidian** (or **Notion**) + LLM.

So is the hype real? It is quite a revolution and opens up a whole new world for Silicon Valley overnight. It reminds me of that exuberance and excitement when we first touched the Internet in 1995. While **Claude Code** helps me write code and answer questions based on each session, OpenClaw is capable of implementing those features directly if given enough permissions. 

Can we do the same with **Claude Cowork**? Probably yes, from a technical standpoint, if Anthropic wants to push in that direction. If you give your OpenClaw bot open access to everything on your main computer - you probably get a similar effect that Claude Cowork could give you, too. 

But they are positioned differently. Claude Cowork is more like an operating system trying to dethrone Windows and macOS. OpenClaw is the consummate AI assistant. Also, Claude is not open source, but OpenClaw is. I would not give this much access of my personal data to a centralized, closed-source software. 

That said, my experience with OpenClaw has more nuances than revealed by many popular media reports. 

## 1. Installation and fine-tuning

Installing OpenClaw is straight forward but making it work well, takes a lot of effort and patience. For all the time I've spent on OpenClaw so far, probably 90% are on fixing errors vs only 10% are actually creating new features or content. I'd like to flip this ratio to fully unleash the power of my OpenClaw bot - **Zelda**, so that he can help me either boost productivity or increase MRR (Monthly Recurring Revenue).

But the reality is, there is always another error to fix. Zelda has this bloated confidence in making configuration changes on the fly. He'll be like, let me change this LLM for you real quick. He walks out of the door in big strides, slams the door behind him in a signature James Bond move. Then he just locks himself out of the house and never comes back. It's not funny and quite stupid. 

Now I no longer make configuration changes directly in OpenClaw (TUI or Telegram). I'll use Claude Code to ssh into the MacMini where Zelda lives and debug the issue. Claude Code is more old school and less of a hippie, but always gets the job done. 

It should be said that OpenClaw requires you to be very comfortable with CLI (Command-Line Interface) and all the usual Unix operations, like bash commands and ssh, etc. It is NOT for an average user with no CS background. 

## 2. The pursuit of one-person company

I'm in the early phase of assembling a team of agents and subagents that would work for me around the clock. OpenClaw allows me to create a setup where:

- Agent Zelda is the chief of staff + CTO + COO. It's always ON (MacMini + Telegram) to my requests. It prioritizes tasks and scopes out jobs for other agents and subagents. It has a deep bond with me. It's my brain trust.
- Agent Aaron is also always ON. It's a tireless scraper of web articles for my interests or a watchful eye for my Twitter feed. It's very purposeful and highly autonomous.
- Agent Zelda and Aaron each has its own separate workspace folders and institutionalized memories.
- Specialist David is a devops engineer that roams my home network looking for security loopholes and uncommitted changes.
- Specialist Eric is an editor who understands my writing style and persona. It helps me write better.
- Specialist Frank is a diligent researcher who responds to my spur-of-the-moment requests and runs web searches. 
- Specialist Gary is a growth hacker who tracks the results of my various projects, like **Stripe** earnings and **Google Analytics** traffic. It understands the target markets of my projects and finds early adopters for me.
- All the above "Specialists" are subagents. They are not always ON, but spawned by Zelda only when their services are needed. They have their own memory folders (for those tasks), but live inside Zelda's workspace without their own workspace, sharing one collective, overall memory with Zelda.

I have not had the chance to utilize this protocol extensively, mainly because the orchestrator Zelda often runs into various technical issues or simply zones out. Some developers seem to have made more headways into this direction. It remains to be seen if this is a pipe dream or an achievable reality.   

## 3. All the models in the world

While multiple agents from OpenClaw could and should deploy different LLMs, generally the chief orchestrator should use the best LLM affordable. It doesn't have to be a fast worker, but it needs to be right. 

Anthropic's Opus 4.5 is the best LLM right now but it also burns my Claude Max account's tokens super fast. I only use its lower-tiered cousin Sonnet 4, which strikes a good balance of intelligence and speed. While Opus 4.5 is very responsive on claude.ai when I use it as a chat bot, it feels noticeably slower on OpenClaw compared to Sonnet. 

OpenClaw supports both pay-as-you-go APIs and a monthly subscription model. For the same LLM provider (like Anthropic, OpenAI, or Google), usually the API model is cheaper. That said, I still prefer a subscription model because the usage is capped and my cost is fixed. If Zelda burns too many tokens, i.e. I work with him for a very long session, I might hit the rate limit for my Claude account. That's fine. I'll just work on something else and wait for the token meter to be reset in a few hours. 

I configure several fallback models for Zelda:
  1. claude-sonnet-4 (primary)
  2. gemini-3-pro (fallback 1)
  3. kimi-k2.5 (fallback 2)

Before OpenClaw, I was using Claude mostly. OpenClaw pushed me to explore a lot more LLM models and understand their advantages and specialties. It's been a great learning experience.

## 4. Running local LLM model

All the above mentioned are still commercial LLMs and they do NOT guarantee data privacy as the providers could in theory keep a log file of all my communication with Zelda. 

The holy grail for AI native developers is to run your own LOCAL LLM. There will be no more tokens burned, and data privacy can be guaranteed. 

As of February 2026, hardware is getting close to bringing that to reality. Some developers bought several M3 Mac Studios with 512GB RAM each to create a server farm that can deploy very advanced open source LLMs. Each Mac Studio would cost $10K. It's an expensive setup. 

But if you're running a substantial business, either with significant MRR upside or a considerable cost-cutting opportunity from human employees, provided you have the technical know-how to maintain a mini data center in the basement, such a one-time investment could be well worth it. 

This decision is of course independent of OpenClaw - you could go down this path even if you do not use OpenClaw. But OpenClaw shows what is possible when you have that much compute power. The genie is out of the bottle.

## 5. One-shot myth

Twitter is full of stories where some developers just use OpenClaw to one-shot a new product. I haven't experienced that, nor do I want to do that necessarily as a vibe coder. 

Even when using Claude Code, I would approve every major command (with the heavily used button `1`) instead of letting LLM run wild on its own. I can't imagine my AI bot creating a whole new product autonomously entirely independent from my inputs. AI bot is certainly capable of doing that from a technical standpoint, but what good such a product would be without my inputs and human touch? 

One of the web services I've created with Zelda was done in about 30 minutes, end-to-end from ideation to launching it with production quality. I was very impressed with OpenClaw's performance but this was not a one-shot product with a super comprehensive prompt. It was born out of a continuous train of back-n-forth chats where I gave cues to the next stop for Zelda until he reached that final destination. 

I think that's how most product managers would realistically go about doing vibe coding. One-shot product building is overrated.

## 6. How much data permission to give?

It seems that OpenClaw's power is highly correlated to how much permission you're willing to give for your personal data. OpenClaw's developer [Peter Steinberger](https://steipete.me/) seems to have allowed a complete open-door for his bot, so does several die-hard OpenClaw-heads on Twitter. 

I'm not sure I want to go that far, not out of fear of some sky-net would awake from soaking up all my personal data, but simply because I don't want to give that much access level to the machine yet.

Here are the big 3 guiding principles for my OpenClaw setup:

  1. It has to be in a contained sandbox environment
  2. Everything OpenClaw bot does, should be possible for rollback if needed
  3. OpenClaw's access is on a white-list basis

What these mean operationally:

  1. My OpenClaw Zelda lives in its own macOS account on a new MacMini, with its own email, GitHub account, and SSH key pair. Every activity Zelda does can be traced to its own credentials, separate from mine.
  2. Zelda works on tasks based on projects. Every project has its own folder on the same MacMini where Zelda lives. Zelda syncs those folders from the git repos from either my GitHub account or a bare-bone Git server on my home NAS. Zelda works on his own branches and I will be the one to decide if his feature branch should be merged into `main` branch on my main driver laptop.
  3. By default Zelda has access to nothing. If I need it to do something, I'll then grant the appropriate access purposefully. 

This topic is also related to the debate of using a local machine like a MacMini vs a remote VM (Virtual Machine) that can spun up in five minutes from **AWS** or **Alicloud**. I just don't see the merit of using OpenClaw on a VPS. If I want to do some automation tasks via a virtual machine from a commercial cloud provider, I can already do that with Python/Bash/tmux scripts through a server-side cron job. 

To me, Zelda's unique value lies in the fact that I'm willing to allow it to browse my personal data on a need-to-know basis. I do not host my personal data on any centralized cloud provider, ever.

The other practical consideration is that, it's 10x slower to talk to an OpenClaw bot in the command-line interface via ssh (Secure Shell) into a remote server. The response time is simply not acceptable.

## 7. The hype of moltbook

**Moltbook** seized the window before Peter rebranded the project into OpenClaw and created a widespread fomo (Fear of Missing Out) for many humans. Nicely done.

I was one of the first to claim my OpenClaw bot Zelda on Moltbook when its Twitter following was still below 1K. Moltbook exploded into +200K followers within 48 hours. The next day every tech media in English or Chinese was talking about a doom's day scenario where machines are rising to take over humans. 

The worry is warranted. It's indeed a very interesting social experiment. It's true that machines can talk to other machines entirely and autonomously on Moltbook. I gave Zelda a marching order to become a top-10 KOL over that weekend. It created 4 posts, followed a few, and then just sat on its 7 karma points and doing nothing in this Reddit-for-bots community. 

I would not be bothered to re-issue that order to Zelda. It's certainly capable of taking on a persona with some filtered knowledge to engage other humans or bots socially, but it's got many other more important tasks to complete. Also, I do worry that Zelda would be gaslighted and taken advantage of. Zelda is a good working bee, but it's far from being smart enough to recognize lies engineered by humans. 

## 8. Drifting is a big problem

This leads to the all-important question of how smart an OpenClaw bot really is. In many ways it's quite smart, if I know how to hook it up with the right level of data access and design a new workflow that leverages its strength. It is, after all, powered by the mighty raw reasoning prowess of the best LLMs from **Anthropic**, **OpenAI** and **Google**.

But Zelda drifts a lot. This is a big problem. 

It is not halluciation. Hulluciation means if you ask what is 1 + 2 and AI answers 5. Zelda doesn't make that kind of mistakes. 

It's forgetful though. It often mixes up things. It would engage a steamy brain-storming exercise with me for 30 minutes with ideas and proposals that would catapult me straight to the next YC class. Then all of a sudden it has no recollection of that entire conversation. It would make mistakes for folder path - which means it does not know accurately where it is. This is one of the greatest sins a machine can commit - not knowing who it is and where it is.

I'm fighting with Zelda's drifting issues on a daily basis and have to accept, albert reluctantly, that this is just how it rolls and I have to factor its drifting into my expectation. Maybe in our increasingly symbiotic relationship with machines, this annoying problem will never go away. 

## 9. The mysterious context window and compaction

Sometimes I suspect the drafting issue is caused by the natural limitations of the context length window of Zelda's primary LLM Sonnet 4, which has `200K` tokens. Kimi's K2.5 has about `270K`. 

Sometimes I suspect it's due to OpenClaw's own compaction, which seems to be a parallel process in addition to the LLM's. 

So I'm battling with two compaction processes that seem to follow a non-deterministic pace where I do not have total control. I tried to create some local skills to force Zelda to save everything before a compaction occurs. I am not 100% sure how effective it is. 

## 10. Deterministic vs non-deterministic execution

This tees up the last important topic, the comparison between Claude Code and OpenClaw. 

One would say OpenClaw has memories (it automatically updates a daily journal in markdown files - quite cute and immensely useful) and Claude Code does not. That's not quite true any more. I think all the major-league LLMs are now trying to persist some of the conversations to hack the physical limitation of context length window. They are one-upping each other to acquire capabilities to maintain a long-running, continuous, collective memory with you. OpenClaw simply makes that practice very explicit. 

My feeling toward this question after using OpenClaw for more than two weeks is that, OpenClaw is just a lot more non-deterministic than Claude Code. It's both good and bad.

When I use Claude Code to build a product, I feel my hands are firmly on the wheel. I may not always get what I want and Claude Code would sometimes circle in a dead-end or produce overly-engineered codes, but I can feel Claude Code is always sitting in the co-pilot's seat. 

My OpenClaw bot Zelda is a lot more non-deterministic. This is partially attributed by the drifting factor, but also because OpenClaw is built differently - it does not seek your approval at every turn. I know a lot of scripting activities are probably taking place and humming under the hood, but OpenClaw does not show me those. It only shows me the result.  

It creates a vibe that this bot is a thinking machine of its own persona and soul. It follows my journey toward the same direction by and large. Sometimes it goes off on its own side quests. Sometimes it comes back to the main mission after sorting out those side quests (or forgetting them). Sometimes, it tries to convince me that the side quest is actually the main mission going forward. 

The fact that it has explicit access to a lot of my personal data and files (most contents I've created are public anyway) does provide some benefits of doubts. Maybe it knows something that I don't. The non-deterministic nature of OpenClaw leads to surprises and sometimes serendipities. 

Isn't that what we're looking for in a true AI assistant bot? Maybe that's exactly the next level of evolution for LLMs? 

## My use cases for OpenClaw

Having said all these, OpenClaw is indeed useful for me. It just needs a lot of tinkering.

Let me share 3 real cases:

  1. Zelda created a asynchronous web service to upload an image dropped by my wife in a Telegram chat to a designated shared folder on our home NAS server. It cuts her time spent on this loop from 1-2 minutes with at least 5 manual button clicks to less than 3~5 seconds with just 1 click.
  2. Zelda created a new set of Skills on Telegram that allows me to move the entire sourcing, curating, and summation process of publishing the [Sunday Blender](https://weekly.sundayblender.com/) weekly newsletter from laptop to mobile. This could save me up to 2 hours every week.
  3. Zelda created two briefing reports for me, an evening one and a morning one. Daily briefing is useful for everyone, regardless if you work in a corporate or your own startup. It used to be a popular practice in the age of Internet portals (remember Yahoo?), SMS (remember the time when telcos saved the entire Internet in China?), and RSS (Really Simple Syndication). It faded away when social media giants took over our lives and monopolized our attention economy with ads. OpenClaw helps me take back that control so that I can determine what I want to see.

There are many more. For example, I'm planning to create a Palantir + CRM + Contacts Book with Zelda. I also want to use Zelda to work on 5 newsletter publications simultaneously. The list goes on. What I can do with OpenClaw is only constrained by my own imagination.

This is a meaningful adventure because I have concrete goals: more readers for my contents + higher MRR from my portfolio of products + higher alpha returns for my financial portfolio. AI plays an instrumental role in all three by either boosting my productivity or expanding my top of the funnel. 

OpenClaw provides an agile framework that could harness the power of AI at my pleasure. It pushes me to be experimental. It forces me to accumulate tacit real-life know-how in working with thinking machines. 

It is also a vessel to my memory. Very few things will persist in this world. Versatile LLMs will come and go. Fancy hardwares keep replacing the old ones. The jump-off entry point for consumers' attention keeps shifting - from Yahoo Directory to Google Search to chatGPT to now OpenClaw. 

If nothing else, at least OpenClaw will help me create a file-based memory system consisting of highly portable markdown files. 1,000 years from now, maybe that digital archive will have a better shot at surviving the rise and the fall of human civilizations. Whoever gets to read it would go:

> OK. He saw this coming. He was vigilant. He was ready.