---
title: "Combine Multiple Images Into One PDF"
date: 2025-03-17T02:00:26.099000+00:00
slug: combine-multiple-images-into-one
draft: false
description: "use command-line tool to create a single PDF file from multiple screenshots"
categories:
  - "newsletter"
---

[![](./2cdedc25-6174-43f6-98cf-b52e9c79a9c9_1200x675.heic)](https://substackcdn.com/image/fetch/$s_!a-cs!,f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2F2cdedc25-6174-43f6-98cf-b52e9c79a9c9_1200x675.heic)

We nowadays take many screenshots. Sharing those images is a hassle. It would be easier to merge the images into a single PDF file. This can be done for free with a command-line tool [ImageMagick](https://imagemagick.org/index.php) on masOS, following this original guide from Stack Overflow[1](#footnote-1).

## Take screenshots

On macOS, there are a few ways to take a screenshot:

***Command + Shift + 3*** will take a snapshot in PNG format for the entire screen.

***Command + Shift + 4*** will turn the cursor into an “aim” sign that allows you to take a selected area of the window, rather than the entire screen.

These screenshot images are saved in the ***~/Desktop*** folder.

## Install ImageMagick

ImageMagick is a powerful image-processing tool for command-line use.

Install ***ImageMagick*** with macOS’s popular package manager ***brew***.

```None
$ brew install imagemagick
```

## Create PDF

First, rename the images with serialized standard names:

[![](./3d403a57-0040-4adc-868b-98c55282f93b_817x91.heic)](https://substackcdn.com/image/fetch/$s_!ipG4!,f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2F3d403a57-0040-4adc-868b-98c55282f93b_817x91.heic)

Then move them into a separate folder. Enter that folder. Convert them into a single PDF file. Use ***\**** sign as a wild card to select all the images.

> **$ convert demo\*.png demo.pdf**

There you go. Hello, ***demo.pdf***!

[![](./7f8c0a6c-f2a4-4e96-a081-14ed5bbd56ce_822x111.heic)](https://substackcdn.com/image/fetch/$s_!ihvz!,f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2F7f8c0a6c-f2a4-4e96-a081-14ed5bbd56ce_822x111.heic)

Now you can share this demo.pdf with your other devices or another friend. It’s much easier to deal with than a bunch of image files.

---

Digital Sovereignty Chronicle is a reader-supported publication. To receive new posts and support my work, consider becoming a free or paid subscriber.

[1](#footnote-anchor-1)

<https://stackoverflow.com/questions/8955425/how-can-i-convert-a-series-of-images-to-a-pdf-from-the-command-line-on-linux>
