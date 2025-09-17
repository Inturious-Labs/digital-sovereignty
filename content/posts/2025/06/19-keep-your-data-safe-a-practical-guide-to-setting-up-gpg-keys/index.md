---
title: "Keep Your Data Safe: A Practical Guide to Setting Up GPG Keys"
date: 2025-06-19T12:07:52.098000+00:00
slug: keep-your-data-safe-a-practical-guide-to-setting-up-gpg-keys
draft: false
images: ["vault.jpeg"]
description: "a step-by-step guide to use GNU Privacy Guard protocol to securely send and receive messages/data on the Internet"
categories:
  - "Digital Life"
---

![hero](./vault.jpeg)

**GPG** stands for GNU Privacy Guard. It is a free and open-source implementation of the OpenPGP standard, used for encryption and signing of data and communications. **PGP** is “Pretty Good Privacy”, an encryption program that provides cryptographic privacy and authentication for data transmission. PGP was originally created by Phil Zimmermann in 1991.

A GPG key ID is required for setting up password manager such as [Pass](https://www.passwordstore.org/) and is generally a good security enhancement for using sites like [Github](https://github.com/), among many other privacy-sensitive services.

The below guide works for mac machine and assumes you already have installed [brew](https://brew.sh/) the package manager for macOS.

Open up **Terminal** app on macOS (or its better replacement, [iTerm2](https://iterm2.com/)) and install GPG on your mac.

```None
brew install gnupg
```

Let’s create the GPG key by running this command.

```None
gpg --full-generate-key
```

This will open up a prompt menu:

```None
gpg (GnuPG) 2.4.8; Copyright (C) 2025 g10 Code GmbH
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Please select what kind of key you want:
   (1) RSA and RSA
   (2) DSA and Elgamal
   (3) DSA (sign only)
   (4) RSA (sign only)
   (9) ECC (sign and encrypt) *default*
  (10) ECC (sign only)
  (14) Existing key from card
Your selection?
```

RSA has been around for a long time and is good enough, but **ECC** (Elliptic Curve Cryptography) is newer, more secure. Pick the default option **9** to use ECC for both signing and encryption.

```None
Your selection? 9
Please select which elliptic curve you want:
   (1) Curve 25519 *default*
   (4) NIST P-384
   (6) Brainpool P-256
Your selection?
```

These are different types of ECC curves. Go with the default (1) **Curve 25519.** It’s a fast, secure, and modern curve that is widely supported in Signal, SSH and Wireguard. NIST P-256 is more for compatibility with government standards. Brainpool P-256 may provide higher level of security but not as widely supported as Curve 25519.

```None
Your selection? 1
Please specify how long the key should be valid.
         0 = key does not expire
      <n>  = key expires in n days
      <n>w = key expires in n weeks
      <n>m = key expires in n months
      <n>y = key expires in n years
Key is valid for? (0)
```

You can pick an expiration window. For now, let’s just go with **0** (key does not expire).

In the next three Terminal prompts, GnuPG needs you to fill three fields: Real Name, Email and Comment.

For Real Name, you can go with … your real name like John Doe, or however you want this key to be identified with.

For Email, use an email that is associated with your above persona. If you want to use this GPG key for Github, make sure the email is verified in the Github account.

For Comment, use something short as a differentiator, like “Personal”, “Work”, “Social”, “Project ABC”, etc.

```None
GnuPG needs to construct a user ID to identify your key.

Real name: John Doe
Email address: john.doe@proton.me
Comment: personal
You selected this USER-ID:
    "John Doe (personal) <john.doe@proton.me>"

Change (N)ame, (C)omment, (E)mail or (O)kay/(Q)uit?
```

Enter **o(kay)** to confirm the choice.

![](././52c7d025-cb69-46c3-91a7-67080f8ad0da_1081x534.png))

This is the fun part. You need to set up a passphrase for using this GPG key. Passphrase is **DIFFERENT** from password. While password should be randomized, specific to each website/application, and impossible for human brain to memorize, a good passphrase should fulfill this set of criteria:

* It does **NOT** appear in any digital form, anywhere, on any device
* **YOU** are the only human that can memorize this passphrase
* It is **EASY** for you to remember
* It’s **COMPLICATED** enough to withstand a brute force hacking attack

If you don’t complete this step in a given time window, the prompt will disappear and you’ll need to start the process from the beginning. GPG protocol will generate a pair of public and private keys and print out this. Let’s show the keys!

```None
gpg --list-keys --keyid-format=long
```

```None
pub   ed25519/95DB481447FBEA2E 2025-06-19 [SC]
      1F18E334416AE571091FA35395DB481447FBEA2E
uid                 [ultimate] John Doe (personal) <john.doe@proton.me>
sub   cv25519/5CAA6F815E17DC9B 2025-06-19 [E]
```

Your GPG **key short ID** would be **95DB481447FBEA2E**, the string behind `ed25519/`. It’s the 8-character (hexadecimal string) unique identifier for your public key.

Your GPG **key long ID** would be **1F18E334416AE571091FA35395DB481447FBEA2E**, which is the 16-character identifier for your public key.

Both IDs can be used. Short Key ID has a slightly higher chance of collision, but it’s good enough for most day-to-day cases and easier to use.

Display your GPG public key in ASCII format (usually for exporting to .**asc** file or copy/pasting that to a server):

```None
gpg --armor --export 95DB481447FBEA2E
```

There you have it. If you need to store your GPG public key to some server like Github or Proton Mail.

```None
-----BEGIN PGP PUBLIC KEY BLOCK-----

mDMEaFPLAhYJKwYBBAHaRw8BAQdAx2RM7v20r6ZaP3eEmYHkB6pVjKBijfN64cFV
F4nPclm0KEpvaG4gRG9lIChwZXJzb25hbCkgPGpvaG4uZG9lQHByb3Rvbi5tZT6I
kwQTFggAOxYhBB8Y4zRBauVxCR+jU5XbSBRH++ouBQJoU8sCAhsDBQsJCAcCAiIC
BhUKCQgLAgQWAgMBAh4HAheAAAoJEJXbSBRH++ou3CkA/RYI9EX9M5Te/Bx+yDGS
qFFB93TdxPmG4smUsmLZTbOqAP9500mVPqWXjhrIjSdMi0VvfM7DCNmDB98pJCsR
EABdDrg4BGhTywISCisGAQQBl1UBBQEBB0BPgb91gTEQOKtUj3zqqgsbGpDm6NJD
vsUbHXUL2EfOKQMBCAeIeAQYFggAIBYhBB8Y4zRBauVxCR+jU5XbSBRH++ouBQJo
U8sCAhsMAAoJEJXbSBRH++ouPqwBAM6msg1lE5tOGqLp/HP4x9uuurqF7WmCSsf8
KGDyFZGBAQCN1fTlhQ9JvcythP9q9oRZhTaHeMnOi+FEkEzMIYWGDA==
=xgWP
-----END PGP PUBLIC KEY BLOCK-----
```

Dada! Congratulations! This is the first step toward your digital sovereignty.

***Useful References:***

<https://docs.github.com/en/authentication/managing-commit-signature-verification/generating-a-new-gpg-key>

---

Digital Sovereignty Chronicle is a reader-supported publication. To receive new posts and support my work, consider becoming a free or paid subscriber.
