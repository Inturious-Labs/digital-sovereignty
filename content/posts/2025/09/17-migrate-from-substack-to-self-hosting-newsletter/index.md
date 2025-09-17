---
title: "Migrate from Substack to Self-Hosting Newsletter"
date: 2025-09-17T14:18:45+00:00
slug: migrate-from-substack-to-self-hosting-newsletter
draft: true
description: "detailed steps of migrating newsletter from Substack to a self-hosting technology stack that gives you complete control at every step"
categories:
  - "Digital Life"
images: [""]
keywords: ["Substack", "Hugo", "dfx", "ICP", "canister", "Internet Computer", "Buttondown", "Stripe", "RSS", "Claude Code", "Cursor", "AWS"]
---

<!-- Featured image for social media -->
![Featured Image](./featured-image.webp)

[Moving your newsletter away from Substack](https://digitalsovereignty.herbertyang.xyz/p/why-i-left-substack/) to a self-hosting platform that gives you total control is not an easy journey. But it can be done, especially with AI's help, from Cursor to Claude Code. I'm very glad that I made the switch. Here's how you can do that, too.

## Step 1: Build your own frontend

1. Pick a hosting service to deploy your newsletter website. I use a canister from the Internet Computer ("IC"), which is a decentralized version of Amazon Web Services (AWS) and one of the leading L1 blockchains. Running your website on an IC canister is a lot cheaper, easier and more secure than deploying that on a traditional cloud platform like AWS, Replit or Vercel. 
2. Pick a frontend framework for the website. The usual choices include React, Vue, Angular and Svelte. In 2025, the most popular framework is [Hugo](https://gohugo.io/). Its build time for this site ranges from 50~100 milliseconds and my deploy script on Github often finishes in less than 30 seconds. It's extremely fast. It renders Markdown files into static HTML pages. Substack's lack of Markdown support is a major turn-off for me.
3. Create a scaffold project with Hugo and dfx (the command-line interface for the Internet Computer) and initiate that as a git repo.
4. Create a new dfx identity and export its private key to the Github repo for CI/CD action script.
5. Create a new canister with the dfx identity. 
6. Pick a theme for the Hugo site. I like [diary](https://github.com/AmazingRise/hugo-theme-diary) and [paper](https://github.com/nanxiaobei/hugo-paper). Both are free. 
7. Build a GitHub action script to deploy the website in the IC canister  automatically upon triggers from `git push` into the `main` branch. 
8. Download all the posts and emails from Substack.
9. Based on the content structure of the chosen frontend framework and the theme, reformat the downloaded HTML files into Markdown ones, and add the appropriate frontmatter. 
10. Make sure the URLs for the posts are consistent with the published articles from Substack, so that you will not lose the SEO ranking.
11. Adopt `page bundling` by grouping all the embedded images into the same folder as the article. Use `imagemagick`, a powerful command-line tool, to reformat images if necessary. Convert HEIC images into Google's webp format (HEIC from Apple is not widely accepted and creates many compatibility issues). Set a maximum width of 1200px. Make sure no image should exceed 500KB in size.
12. Set up `categories` or `tags` if needed. I like category better because it requires the writer to make only one decision. Tags are mind-boggling. As a writer, I'll have to make way too many decisions that will just drag me down.

## Step 2: Configure custom domain

13. 

## Step 3: Set up email delivery

## Step 4: Create RSS feed

## Step 5: Import email list

## Step 6: Integrate Stripe

## Step 7: Improve SEO

---

*Published in [Digital Sovereignty Chronicle](https://digitalsovereignty.herbertyang.xyz/) - Breaking down complex crypto concepts, exploring digital sovereignty, and sharing insights from the frontier of decentralized technology.*
