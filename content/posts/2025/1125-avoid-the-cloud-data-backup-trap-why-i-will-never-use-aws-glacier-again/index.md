---
title: "Avoid the Cloud Data Backup Trap: Why I Will Never Use AWS Glacier Again"
date: 2025-11-25T12:00:00+08:00
slug: avoid-the-cloud-data-backup-trap-why-i-will-never-use-aws-glacier-again
draft: false
description: "Amazon' low-cost storage service AWS Glacier is a nasty parasite. It's very difficult to be killed off. It sucks your blood dry."
categories:
  - "Digital Life"
images: ["hero.jpeg"]
keywords: ["AWS", "AWS Glacier", "Data Backup", "Synology", "NAS", "Claude Code", "Substack"]
enable_rapport: true
---

<!-- Featured image for social media -->
![Featured Image](./hero.jpeg)

Data backup is a complex task. I have long been practicing the so-called **3-2-1** strategy: **three** copies of the data stored on **two** different media types with **one** copy kept off-site. 

I took the whole "one copy offsite" to the extreme by activating **AWS Glacier** service in 2018. AWS Glacier is Amazon Web Services' low-cost storage service for data archiving and long-term backup. It keeps a backup copy of your data in remote locations and assumes you will rarely need to access that on a daily basis. So the storage cost is extremely low. If you do need to retrieve the data, you pay a fee to get the archive. For someone who prizes data sovereignty, this sounds like exactly what I need.

That is, until I found out what a nightmare it is.

I used two regions from AWS Glacier that are far from each other and my current location, the **Tokyo** region and the **Ireland** region. They were set up to sync with my home **NAS** (Network Attached Storage) Synology 918+ and generated archives for all my important files (photos, videos, docs). 

A few months ago, I wanted to stop this service. With the rapidly declining cost of **SSD** (Solid State Drive), storaging large volume of data on a portable SSD hard disk became economical and fairly easy. Keeping an archive on the far end of Europe and Asia seemed an overkill. Its monthly bill of +$30 could be saved to pay for much more exciting AI services like [Google Gemini 3](https://digitalsovereignty.herbertyang.xyz/p/the-end-of-human-podcast-host-the-menace-of-google-notebooklm/). 

Then I hit a stonewall. It was very difficult, if not impossible, to stop AWS Glacier service. I cannot shut it down.

Initially, I thought this would require no more effort than clicking a few buttons to stop the service and call it a day. No, it doesn't work like that. AWS says my vaults were not empty. The vaults have to be emptied first before they can be deleted. If vaults are not deleted, AWS will continue to charge the monthly fee.

Emptying my vaults turned out to be a considerable technical endeavor. 

For a starter, AWS does NOT provide a GUI (Graphical User Interface) to perform this job. I have to use AWS' CLI, or Command Line Interface. Actually, this was the reason I gave up this same effort in prior years. I didn't have the time or patience to learn how to use another CLI. Also AWS' instructions were very technical and confusing.  It seemed to be an overwhelming task and I was scared off.

I've grown into a much more capable developer in 2025, with the help of **chatGPT**, **Cursor**, and **Claude**. Thankfully, I can just throw that to **Claude Code** and ask Claude to guide me through the process. CLI is no longer an obstacle. 

My full-stack CTO partner **Claude Code** does help me make some sense out of it. I have `745,741` archives of `7.46` TB data stored on the two regions. I need to first download an index table for all the archives and then delete every single archive from that table recursively. Claude can write up a Shell script to execute this task, and it will take `2-3 weeks` of running the script non-stop to complete this task. 

Ah, I remember now, this was the second obstacle that stopped me in the past (I was on this side quest multiple times). I got this far last time on my own, even without Claude. I still had to give up because I simply could not let my laptop keep running for 2-3 weeks without interruption. I traveled too much. 

No worries. My new partner Claude can find another way. I'm running an **Ubuntu** virtual machine on my Synology 918+. Maybe I can execute this script NOT on my laptop, but on the Ubunbu server? 

That is indeed the right path forward. Claude helped me update good old Ubuntu from version 18 to the latest version 24.04 (this task itself is a major undertaking); get rid of Gitlab server (which is eating up a lot of memory); set up the passwordless login with SSH; configure the bare-bones git remote on Synology; and figure out how to send email to myself from the server.

I ended up with `10` scripts of `1,236` lines of Python and Shell scripts working for me. I started a `tmux` job on Synology to loop through the archive table and delete file by file on the Tokyo and Ireland regions. Every 12 hours, Synology will send me an email giving me an update on the progress, like this one on Oct 25:

```
AWS Glacier Deletion Progress Report
Generated: 2025-10-26 06:00:02 CST

Overall Progress: 12,468 / 745,737 archives (1.67%)

============================================================
Tokyo (KunLun_001132B7836D_1)
============================================================
Status: Running
Progress: 6,207 / 475,606 archives (1.31%)
ETA: 27d 21h 19m (around Nov 23, 03:19)
Last Activity: 2025-10-26 06:00:01

============================================================
Ireland (KunLun_0011324E665A_1)
============================================================
Status: Running
Progress: 6,261 / 270,131 archives (2.32%)
ETA: 15d 13h 0m (around Nov 10, 19:00)
Last Activity: 2025-10-26 05:59:57

Overall ETA: 21d 16h 31m (around Nov 16, 22:31)


✅ Deletion processes running normally

The deletion will take approximately 5-6 days total. You'll receive progress
updates every 12 hours. No action needed unless you receive an error alert.

To monitor live: ssh dalaran && tmux attach -t tokyo  (or ireland)
Press Ctrl+B then D to detach without stopping the process.
```

On one hand, I felt very pumped up for the newfound superpower endowed by Claude Code.  I've made far more progress on this quest than in any of my prior attempts. Kudos to Claude! AI is a real game-changer for a junior developer with a shallow technical background like me. This whole automation workflow is pretty sleek. I feel my hands are on the wheel, in control.

On the other hand, this is actually STILL not good enough to kill Glacier. Here's the most recent one from this morning:

```
AWS Glacier Deletion Progress Report
Generated: 2025-11-23 18:00:03 CST

Overall Progress: 729,070 / 745,737 archives (97.77%)

============================================================
Tokyo (KunLun_001132B7836D_1)
============================================================
Status: Running
Progress: 458,939 / 475,606 archives (96.50%)
ETA: 1d 1h 9m (around Nov 24, 19:09)
Last Activity: 2025-11-23 18:00:01

============================================================
Ireland (KunLun_0011324E665A_1)
============================================================
Status: Stalled (no activity for 10+ minutes)
Progress: 270,131 / 270,131 archives (100.00%)
Last Activity: 2025-11-15 14:43:40

Overall ETA: 0d 15h 50m (around Nov 24, 09:50)


⚠️ WARNING: Deletion process may have stalled!
```

Something is off. My script tells me all the `270,131` archives on the Ireland region have been deleted, but it still could not execute the next action, which is to delete the vault, because AWS kept saying the vault was (still) not empty. AWS' web dashboard told me there were still `72` files left in the vault. I restarted the script, and it stalled again, back to the same place. The script is trapped in a perpetual Catch-22. 

This feels very similar to another recent episode I encountered after migrating my newsletters away from Substack only to find out that [Substack continued to serve zombie posts of my newsletter](https://www.reddit.com/r/Substack/comments/1p1rpt8/zombie_posts_from_substack/) to essentially steal Google traffic from my new sites. 

After battling with this Glacier thing for a month, the fight goes on. I have to spend more time with Claude to investigate the cause and find a fix in this never ending saga. In the meantime, AWS sends me another monthly bill.

A friend of similar experience told me that, even if I go with a "nuclear" route by closing out my AWS account for good, AWS will still continue to charge to my credit card. It's a tenacious leech that will not let go of you easily until you're bled to death. 

I understand that AWS doesn't want to make this deletion job too easy with the snap of the finger, maybe in the name of protecting user's important data. But hey, I already passed all the authentication tests. This is **MY** data. I can do whatever I want with it. Why do you want to make this SO difficult for me?

That's because Amazon is a for-profit company. Every product decision is optimized for extracting maximum revenue from users. Data is everything in the age of AI. It will do whatever it takes to stop its users from moving data away. It is holding its users at ransom, because it has that much power and can get away with it. 

It's not just AWS, but also Substack, Meta, Google, Microsoft, Tencent, Alibaba, ByteDance, the lot. 

You can use the tools from them, but don't pledge your data to them. Your data will define what you are and who you are. If you have no control of you data, your life will leave no digital footprint 100 years from now, in the all-seeing eyes of chatGPT and DeepSeek. For all intents and purposes, even for your descendants, you never existed. You leave no legacy behind. The world continues to roll forward with no verifiable contribution from you.

You might have lived a life, but you cannot prove that. That will be the greatest peril for man.

---
