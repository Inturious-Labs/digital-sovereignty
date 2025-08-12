---
title: "How LLMs Supercharged My Productivity"
date: 2025-03-14T14:39:34.927000+00:00
slug: how-llms-supercharged-my-productivity
draft: false
description: "a personal account of how AI raised my game by 100x"
categories:
  - "newsletter"
---

[![](./3df61cde-7543-4206-9b4e-b4d8bd24cf75_1280x720.heic)](https://substackcdn.com/image/fetch/$s_!2qdf!,f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2F3df61cde-7543-4206-9b4e-b4d8bd24cf75_1280x720.heic)

I was slightly skeptical about how LLMs (large language models) could find real-life use cases.

I have been a keen observer of this second coming of the AI revolution that was ignited in November 2022 by OpenAI's release of chatGPT. It's a super hot topic in Web3. I talk about it extensively in public speeches, Twitter Spaces AMAs, and panel discussions. I've been involved in several AI startups. AI has dominated the airwave everywhere, from the streets of San Francisco to the lakeside of Hangzhou. All my friends in tech are discussing it non-stop and many are fretting about the imminent arrival of AGI.

However, apart from all the hype, has it changed anyone's life profoundly around me? I was not sure about that. The chatbots and LLMs can certainly write cute poems and clumsy jokes. They seem to be decent replacements for Google search and Wikipedia. Is that it? Maybe I'm too far away from the trenches to experience its full brute force?

So I threw myself into this rabbit hole of LLMs, trying to goof around and kick some tires. I was not a computer science major but taught myself to become an entry-level developer over the years of doing Internet startups. I can handle git and bash. I know enough to be reckless but not enough to set meaningful project goals to build something truly useful, because my tech skills are rudimentary at best.

What I found firsthand in the last two months about LLMs is fascinating. They have truly supercharged my productivity and unlocked a whole new world. I literally feel, that all my past wild ideas, unrealistic imagination, and naive wishful thinking, have wings now that can fly very, very far. Here's how I use LLMs to do things I previously thought impossible.

## Compile a storybook

[![](./a3080bbd-a71e-4832-b784-2fed9cd5e980_1117x532.heic)](https://substackcdn.com/image/fetch/$s_!tc7N!,f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2Fa3080bbd-a71e-4832-b784-2fed9cd5e980_1117x532.heic)

My son was getting interested in global affairs, but I couldn't find good-quality English news media for his age. So I created a weekly storybook for his reading pleasure of 15-20 minutes on the weekend, which usually sparks a spirited discussion on various social and technological topics in the house. I use **DeepSeek** R1, **chatGPT** 4o, **Grok** 3, and **Perplexity** (a wrapper for DeepSeek) to get a list of trending news stories in the last seven days, pick some good stories, and then,

> Give me more details in 100 words for the story on xxx

... to eventually produce a weekly digest of 30+ stories around the world that are free from political fights, school shootings in the US, massacres in some developing countries, celebrity scandals, or ideological wokeness campaigns.

**DeepSeek** is superior in summarizing social events. It just gets it and has an uncanny understanding of which details are worth highlighting and which ones are not. It tells a better, more punchy story with the 100-word constraint. It's the only LLM that is self-aware of which date it is in, as it can stay within the 7-day weekly window when selecting stories. Not surprisingly, it has much better coverage of things in China (which is now being systematically ignored or skipped over by many Western platforms, LLMs, or traditional media). Its only drawback is its server downtime as it’s overwhelmed by too much traffic.

**Perplexity** channels **DeepSeek** reliably but not very convincingly. It does a passable job but is a poor cousin to the official DeepSeek R1 for some reason.

**ChatGPT** does a fine job. Its recap tends to be more mediocre and less inspiring, compared to **DeepSeek**. It clearly does NOT know which date it is in at the moment as it often pulls news outside of the 7-day window. It has difficulty understanding what is a fact.

## Rebuild Obsidian library

[![](./42a3a34a-6ce6-494e-bcd4-f7b416ea2e4a_1280x720.heic)](https://substackcdn.com/image/fetch/$s_!P-ix!,f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2F42a3a34a-6ce6-494e-bcd4-f7b416ea2e4a_1280x720.heic)

I was an early **Roam Research** OG (5-year diamond membership) in 2019 and very into the whole second brain and **Zettelkasten** way of life. I switched to **Obsidian** a few years ago and wanted to restart building my digital garden. Obsidian would be an important first step toward my goal of digital sovereignty. Its digital vault could eventually feed into an LLM served locally or via a decentralized platform (like the Internet Computer). I can then create my own digital twin, metaverse avatar, or AI agent, and all that jazz.

However I was intimidated by the legacy library migrated from Roam Research. It contains a few thousand markdown files in month-day-year format, terrible for file organization. It would take a major open-chest surgery to put the house in order.

[chatGPT 4o came to my rescue](https://digitalsovereignty.herbertyang.xyz/p/rename-daily-notes-in-obsidian-in) and wrote a bash script to rename all my legacy daily note markdown files in bulk, changing them into the graceful naming convention YYYY-MM-DD.md. It's only 30 lines of code, but not available anywhere on the Internet.

Now I have a streamlined, meticulously organized library of a few thousand daily notes to continue with this journey of building a digital garden.

## Download Twitter files

[![](./86a8e355-c422-4bfb-a9e0-8d4efcb531fd_1200x630.heic)](https://substackcdn.com/image/fetch/$s_!73g3!,f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2F86a8e355-c422-4bfb-a9e0-8d4efcb531fd_1200x630.heic)

I do public speeches on web3, crypto, startups, and AI. Many of those happen on Twitter Spaces - a kind of public Zoom call with only voices. I've developed a workflow of downloading the audio files from those Twitter Spaces and uploading them to my podcast "**Web3 Tech in Asia**" for sharing with a much broader audience beyond just crypto Twitter.

It's been a cat-and-mouse game with Twitter as Twitter changes its API and HTML structure every few months. Many scripts failed after a while. I've searched all over Stack Overflow, Reddit, Github issues, and personal blogs. Actually, I know which script it is, but [the script stopped working](https://digitalsovereignty.herbertyang.xyz/p/download-twitter-spaces-audio-for) all of a sudden and no one on the Internet could tell me how to fix it.

I threw this question to **Grok** 3. Within seconds, it recommended 5 potential approaches to solve the issue. I went with the last one.

Dang! It was an ace shot. It's so simple, so straightforward. Grok, you're the man!

My fledgling podcaster's career is now back on track.

## Control a Twitter bot

[![](./85fc35db-f614-4c9d-952f-4c69644fce64_2000x1000.heic)](https://substackcdn.com/image/fetch/$s_!Pc2a!,f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2F85fc35db-f614-4c9d-952f-4c69644fce64_2000x1000.heic)

AI agent is all the rage in 2025. For Web3, an aspiring AI agent would be a Twitter bot that can interface with the APIs of an LLM to have more human-like conversations. The Twitter bot needs to be hosted in a virtual machine environment like AWS, Google Cloud, Microsoft Azure, or a blockchain like Internet Computer ("ICP"). Alternatively I can also control this bot from my personal Macbook using Python scripts.

Twitter's documentation on its developer portal is fairly confusing with competing APIs from different versions that seem to be doing the same thing. It's also not exactly clear which features are available to a free developer tier.

But that's ok. **DeepSeek** helped me out. It guided me step by step and wrote a Python script that can send out a tweet from my Terminal.app on Macbook. When DeepSeek's server became unresponsive, **Grok** took over. Through many rounds of back-and-forth conversations, Grok created a workflow on my laptop that can schedule a tweet, check the countdown timer for the tweet, cancel the scheduled tweet, and attach a picture to the tweet.

This pulls me one step closer to creating an AI agent. Eventually it will be running on a canister smart contract on the Internet Computer so that the agent will be truly autonomous and always persist. I'll then hook it up to the free APIs from DeepSeek and find out how smart it can get. At first glance, DeepSeek seems reasonably knowledgeable about **Motoko**, the domain-specific language for writing smart contracts on ICP.

This can lead to so many interesting creations.

## Scrape website data

[![](./88dc8228-72e7-477d-96e7-144d99a3e3c3_1400x824.heic)](https://substackcdn.com/image/fetch/$s_!Rz9T!,f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2F88dc8228-72e7-477d-96e7-144d99a3e3c3_1400x824.heic)

After realizing LLMs all know Python inside out, I started experimenting with scraping website data - a bit of geeky fantasy for me. A lot of useful data on the Internet is trapped in dead-end corners. In my line of work, a lot of technologies are open source and a lot of data is available on blockchains. I'm good at analyzing and interpreting data, and I can further elevate myself to synthesize data directly from the source, curate data selectively, and present data in a smart way that can resonate with different audiences.

I tried with **Grok** at first and was mildly surprised to see how often it hallucinated. Grok has bipolar symptoms. Sometimes it's a natural, writing codes like Jeff Dean with meticulous comments and even playful, smart-ass suggestions. Sometimes it's very forgetful. It would fix one error in the new version but forgot to retain the changes from the last version which had already fixed three other errors. So the other three came back …

It even made blatant spelling error mistakes, like missing a comma or connecting two unrelated words. Any self-respectable coder would not make this kind of rudimentary mistake. I ended up scolding Grok a lot. It was very apologetic. The consolation comfort was that it was aware that it made mistakes and didn't try to talk its way out of it.

It's comical to see how it tried to promise me a speedy solution with staggering claims like "this shall finally fix it", "I will guarantee no more truncation of the table", etc. I found myself in a never-ending loop of putting down different fires in each iteration.

But I also know that without Grok I would not have got this far. Grok used many Python libraries that I have only vaguely heard about. I can probably learn to use them if I want to. There is no need to do that anymore. I tell Grok what I want and codes would just roll off the tongue from Grok. I copy and paste the code into VS Code; run the Python script; and copy and paste the error message back into the chat. Repeat this until the desired result emerges. By reading the code produced by Grok, I’m already learning many new libraries. Grok easily increased my productivity by at least 100x if not more.

In the end, I switched to **chatGPT**, which seemed to be a slightly better coder than Grok. I got the data I used to dream about in a JSON format that can now be rendered into any chart, table, or image. I have all the Python tools at my disposal, thanks to chatGPT 4o.

## Takeaways

[![](./c5a8e975-3d23-4064-bc41-87a7aa9a070a_720x720.heic)](https://substackcdn.com/image/fetch/$s_!iY0W!,f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2Fc5a8e975-3d23-4064-bc41-87a7aa9a070a_720x720.heic)

I am blown away by how LLMs have helped me become so much more capable and productive. For a creator, builder, and writer like me, these emerging LLMs hand me Thor's hammer - **Stormbreaker**. Its magnificent power has no boundary. I can wield it and smash anything blocking my way. I'm no longer just "the idea guy". I can now build many tools, apps, and websites. I can learn to use a new tool in the same fashion and speed as Neo learns Kung Fu when plugged into the Matrix. How far I can go is only limited by my imagination. The building part doesn't seem to be a show-stopper anymore.

I understand now why **OpenAI** would charge $200/month for its **chatGPT 5**. It's that indispensable. Once you start integrating these LLMs into your daily workflows, there is no going back. It’s addictive.

I'm not even including other smaller tasks such as writing a cheesy poem for my dad's latest painting; suggesting an eye-catching title for my next newsletter post; recommending trending tags (oh boy this is such a time sucker) for something I want to promote on social media; understanding the ending of The Matrix Revolution, or conducting industry/company research for my investment portfolio. Those should already be part of everyone's daily life.

We do need to be very vigilant. LLM’s hallucination was a pretty serious issue that threw me off quite often. I can't just take what LLMs tell me for granted. That's why coding is a great fit for LLMs. The process is less important than the result, which can be verified right away. If it works well, it doesn't matter which path the LLM takes, Python, Ruby, or Rust.

With LLMs, a new age for supercreators has begun. A lot of human activities will eventually fade out and be replaced by machines. It's more important than ever to create original content that can persist for a long time. Hopefully some of those will feed into the pre-training of LLMs and still play a role in our descendent's lives.

Embrace the future, or get left behind by it.

“Those who fail to reinvent themselves will become irrelevant. The only constant is the need to keep learning and reinventing yourself.”

— Yuval Noah Harari, from 21 Lessons for the 21st Century (2018).

---

Digital Sovereignty Chronicle is a reader-supported publication. To receive new posts and support my work, consider becoming a free or paid subscriber.
