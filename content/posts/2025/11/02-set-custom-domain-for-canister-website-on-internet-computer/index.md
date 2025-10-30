---
title: "Set Custom Domain for Canister Website on Internet Computer"
date: 2025-11-02T12:00:00+08:00
slug: set-custom-domain-for-canister-website-on-internet-computer
draft: false
description: "guide for how to set up custom domain name for your canister-hosted website deployed on the Internet Computer Protocol"
categories:
  - "Digital Life"
images: ["canister-hero.jpg"]
series:
  - "Self-Host Website"
keywords: ["self-hosting", "DFINITY", "IC", "ICP", "Internet Computer", "canister", "custom domain", "DNS"]
enable_rapport: true
---

<!-- Featured image for social media -->
![Featured Image](./canister-hero.jpg)

After deploying a frontend canister on the **[Internet Computer ("IC")](https://internetcomputer.org/)** - a layer-1 blockchain that allows running a full-stack web service, the URL for the site looks like this:

https://wupbw-2aaaa-aaaae-abn7a-cai.icp0.io/

Though fully functional and providing a cue to validate [this smart contract on the blockchain explorer](https://dashboard.internetcomputer.org/canister/wupbw-2aaaa-aaaae-abn7a-cai) similar to what you could do for a **Bitcoin** address, this is geeky and hard to remember. 

Here's the guide on how to configure the custom domain for your website that is deployed on an IC canister, so that when viewers type https://hello-roddy.xyz, they will be directed to https://wupbw-2aaaa-aaaae-abn7a-cai.icp0.io. All they have to remember is the "normal" (or vanity) domain name `hello-roddy.xyz`, which you purchase from a domain name registrar such as **GoDaddy**.

You can configure an IC canister site to point to not only a domain like `hello-roddy.xyz`, but also a subdomain like `blog.hello-roddy.xyz`. 

## 1. Create DNS records

In the DNS zone of your domain registrar, create 3 records:

If your site is a [root domain](herbertyang.xyz/), `hello-roddy.xyz`:

| Host | Value | TTL (seconds) | Type|
| --- |--- |--- |--- |
| `@` |  `hello-roddy.xyz.icp1.io` | 300 | `ALIAS` |
|  `_acme-challenge` | `_acme-challenge.hello-roddy.xyz.icp2.io`   | 300  | `CNAME`  |
|  `_canister-id` | `wupbw-2aaaa-aaaae-abn7a-cai`  | 300  | `TXT` |

If your site is a [subdomain](https://digitalsovereignty.herbertyang.xyz/), `blog.hello-roddy.xyz`:

| Host | Value | TTL (seconds) | Type|
| --- |--- |--- |--- |
| `@` |  `blog.hello-roddy.xyz.icp1.io` | 300 | `CNAME` |
|  `_acme-challenge.blog` | `_acme-challenge.blog.hello-roddy.xyz.icp2.io`   | 300  | `CNAME`  |
|  `_canister-id.blog` | `wupbw-2aaaa-aaaae-abn7a-cai`  | 300  | `TXT` |

Note that the [official guide](https://internetcomputer.org/docs/building-apps/frontends/custom-domains/dns-setup) says you should create `ALIAS` for subdomain for host `@`. In my experience, ALIAS does NOT work. You have to use a `CNAME` record instead.

## 2. Create dfx specific parameters

Create a file named `ic-domains` and add your domain in this file:

```
hello-roddy.xyz
```

Place this file in a folder `.well-known`, in the dfx root directory, such as:

```
├── .git
├── .dfx
├── dfx.json
├── ic-assets.json5
├── package.json
├── canister_ids.json
├── .well-known
│ └── ic-domains
```

By default, `dfx` excludes all files and directories starting with a `.`. To include the `ic-domains` file, you need to create an additional file `ic-assets.json5` in the SAME directory as `.well-known` folder. 

`ic-assets.json5` file needs to contain this json string:

```
[
  {
    "match": ".well-known",
    "ignore": false
  }
]
```
While the above arrangement works for a simple [semantic HTML site](https://inturious.com/), it won't work right away for a static website built by a **Static Site Generator** (Pelican, React, Docusaurus, Hugo, etc). 

For example, a [Hugo](https://gohugo.io/)-based website's output folder is `public/`, which is re-generated every time Hugo builds. If these three files, `.ic-assets.json5`, `.well-known`,  and `ic-domains` were placed in `public/`, they will be erased every time Hugo regenerates this folder.

So all these three **dfx**-specific files should be placed in `static/` folder, together with other permanent media assets such as site logo and favicon. All the contents in `static/` are copied directly into `public/` when Hugo re-deploys the site. 

```
├── dfx.json
├── package.json
├── canister_ids.json
├── public
├── static
│ ├── .ic-assets.json5
│ ├── .well-known
│ │ └──ic-domains
```

## 3. Deploy the canister

When the above three dfx files are in the right place, deploy the canister.

## 4. Register the domain

Run this command to register the domain with the HTTP gateways:

```
curl -sL -X POST \
    -H 'Content-Type: application/json' \
    https://icp0.io/registrations \
    --data @- <<EOF
    {
      "name": "hello-roddy.xyz"
    }
EOF
```

If the call is successful, you will receive a JSON response that contains a request ID (a very long text string) in the body:

```
{"id":"REQUEST_ID"}
```

## 5. Check the progress

Check the status of your registration request. Usually, you can get a confirmed message within 1~2 minutes.

```
curl -sL -X GET \
    https://icp0.io/registrations/REQUEST_ID
```

When you see `Available: The registration request has been successfully processed.`, congratulations! Now you can access your canister website using your preferred custom domain.

For additional technical details, check out this official IC guide: https://internetcomputer.org/docs/building-apps/frontends/custom-domains/using-custom-domains#custom-domains-using-your-own-infrastructure too, though it's a lot more technical with tons of details you probably don't need to know.

Custom domain configuration for an IC canister-hosted site does require a few extra steps than traditional website operation. But given the significantly enhanced security, total sovereignty ownership, much lower cost, and future ease of site upkeep, this effort is still well worth it.

---
