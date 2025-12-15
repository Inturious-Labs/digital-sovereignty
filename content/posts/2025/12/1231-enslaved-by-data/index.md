---
title: "Enslaved by Data"
date: 2099-12-31T00:00:00+08:00
slug: enslaved-by-data
draft: true
description: "how we are becoming slaves of data"
categories:
  - "Digital Life"
images: [""]
keywords: ["ICP", "Internet Computer", "互联网计算机", "Internet Computer Protocol", "canister", "blockchain", "crypto", "decentralization", "digital sovereignty", "区块链", "Jellyfin", "Synology", "Backup"]
enable_rapport: true
---

![Featured Image](./featured-image.webp)

I was trying to watch the recent Emma Stone movie Bugonia on Jellyfin. Jellyfin is an open-source alternative to Plex that allows you to create your own film/show/music library on the home network and enjoy entertainment on all connected devices. 

But alas, it could not play. Jellyfin seemed to be offline. This is strange. It has been working like a charm for the past two weeks.

With a very capable assistant named Claude, I investigated high and low on the home network to see why the Jellyfin library went offline.

It turned out that the two video folders for Jellyfin were not mounted on the network. Jellyfin couldn't find the path to the video library.

Surprisingly, my Jellyfin server was deployed on my 2018 Mac Mini via the Jellyfin macOS application. Maybe I was thinking about leveraging Mac Mini's 32GB memory and 1TB hard disk when setting up Jellyfin the first time two weeks ago, so I chose to deploy Jellyfin on the Mac Mini instead of Synology 918+, my home NAS.

But now I think more about this, hosting Jellyfin on an always-on by 24/7 NAS is probably a better configuration, so that the video drives will always be mounted to Synology. I will not have to worry about the flaky connection between the Mac Mini and its external drives. 

So I tried to re-deploy Jellyfin on the Synology NAS via Docker, the recommended method from Jellyfin. 

Somehow the current docker.app on Synology could not find the installation file for Jellyfin. This is strange. It used to work fine. I installed a Bitcoin node using this docker method a few years ago. 

Even stranger, Jellyfin's official documentation keeps saying "container.app" for Synology in its setup tutorial, but I only saw docker.app. Are we looking at the same thing?

A bit of Google research revealed that Synology released a major update to its operating system (DSM) two years ago, from 7.1 to 7.2. Part of this major update was to rebrand docker.app as container.app and many related changes. Ah, that must be the reason. The docker.app on my Synology is deprecated. No wonder it could not find Jellyfin. 

Guess I just have to update my Synology's OS from 7.1 to 7.2. But before getting to 7.2, I still have to finish a few smaller updates within 7.1. Sure, let's get all of them updated, once and for all.

A few hours later, I was finally able to press the button on 7.2. Synology prompted me: 

> Please make sure you have backed up all your key data before proceeding with this major update

Hmm, you just have to say this out loud, don't you? It's been a few months since I did a full backup of my Synology. This is supposed to be a monthly routine task, but I have been procrastinating on it. 

Because this backup job was approaching its file size limit. My mobile hard drive has 4TB capacity. I vaguely remember in my last backup, all the family photos and videos, and documents for the past 20+ years were adding up to somewhere close to 3.5TB. 

My wife and I had a soul-searching and philosophical conversation about our current and future usage of data. Our conclusion is that let data be data; the beast is out of the cage and cannot be contained, and we just need to top up the hard disk storage on Synology. The problem is that our NAS' current storage usage is about 80% and doesn't leave much room for more data.

Given the way this crazy AI arms race is going around the world, the cost for data storage is likely NOT going to come down any time soon. So we decided that we should expand the NAS storage.

How to expand the NAS storage is not so straightforward. There are multiple paths: 

- We can stick with the current 4-bay Synology 918+ and replace all the 4 NAS drives from 4TB (each) to 8TB
- We can buy a 5-bay expansion unit to connect to 918+ and need to get a new closet for this massive machine
- We can replace the current Synology enclave with an 8-bay one so that it's even more advanced 

This is too mind-boggling for an otherwise very peaceful and stress-free Sunday afternoon. So I put it aside and attended to Claude to see how much storage I would need to do a backup now. 

Luckily, Claude's analysis showed that I only need less than 3TB. This is well within the capacity limit of my mobile drives. One bullet dodged. 

Finally, I activated Hyper Backup on Synology and started the backup. This is going to take a few days. 

Wait, what was it that I was trying to do in the beginning? I remember there was something about Emma Stone. 



