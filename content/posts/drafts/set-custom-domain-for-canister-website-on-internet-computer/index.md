---
title: "Set Custom Domain for Canister Website on Internet Computer"
date: 2099-12-31T00:00:00+08:00
slug: set-custom-domain-for-canister-website-on-internet-computer
draft: true
description: "guide for how to set up custom domain name for your canister-hosted website deployed on the Internet Computer Protocol"
categories:
  - "Digital Life"
images: [""]
series:
  - "Self-Host Website"
keywords: ["self-hosting", "DFINITY", "IC", "ICP", "Internet Computer", "canister", "custom domain", "DNS"]
enable_rapport: true
---

<!-- Featured image for social media -->
![Featured Image](./featured-image.webp)

After deploying a frontend canister on the **Internet Computer ("IC")** - a layer-1 blockchain that allows running a full-stack web service, the URL for the site looks like this:

https://wupbw-2aaaa-aaaae-abn7a-cai.icp0.io/

Though fully functional and providing a cue to validate [this smart contract on the blockchain explorer](https://dashboard.internetcomputer.org/canister/wupbw-2aaaa-aaaae-abn7a-cai) similar to what you could do for a **Bitcoin** address, this is geeky and hard to remember. 

Here's the guide on how to configure the custom domain for your website that is deployed on an IC canister, so that when viewers type https://hello-roddy.xyz, they will be directed to https://wupbw-2aaaa-aaaae-abn7a-cai.icp0.io. All they have to remember is the "normal" (or vanity) domain name `hello-roddy.xyz`, which you purchase from a domain name registrar like **GoDaddy**.

You can configure an IC canister site to point to not only a domain like `hello-roddy.xyz`, but also a subdomain like `blog.hello-roddy.xyz`. 

## 1. Create DNS records

In the DNS zone of your domain registrar, create 3 records:

If your site is root domain:

| Host | Value | TTL (seconds) | Type|
| --- |--- |--- |--- |
| `@` |  `hello-roddy.xyz.icp1.io` | 300 | `ALIAS` |
|  `_acme-challenge` | `_acme-challenge.hello-roddy.xyz.icp2.io`   | 300  | `CNAME`  |
|  `_canister-id` | `wupbw-2aaaa-aaaae-abn7a-cai`  | 300  | `TXT` |

If your site is a subdomain:




## 2. Create dfx specific parameters

## 3. Deploy the canister

## 4. Register the domain

## 5. Check the progress

---
