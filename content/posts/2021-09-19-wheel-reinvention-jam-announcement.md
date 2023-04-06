+++
title = "Wheel Reinvention Jam - Project Announcement"
description = "Announcement of my project for the 2021 Wheel Reinvention Jam"
# date:   2021-09-19 19:34:00 +0200
+++

*I'm planning to participate in the [Wheel Reinvention Jam](https://handmade.network/jam) organized by the [handmade.network](https://handmade.network/). I'm re-posting the project description I've written to [register for the event](https://handmade.network/forums/jam/t/8074-register_here__jam_projects___teams%2521) here:*

\-

Hi Everyone,

My project will be a re-imagination and generalization of text editors. It will be changing the definition of a text editor from "a program for editing text files" into "a program for editing any file that can be converted into a textual representation". 

Text editors have been around for decades and are still popular for good reasons. Editing content as plain text is an easy and natural interaction with a computer. The files text editors work with are text files, basically a sequence of characters. We store a lot of content in text files, mainly because this allows any text editor to display and edit them. That's a huge benefit compared to other formats that might need specialized software for editing them.

On the other hand, text files have quite some shortcomings: 

First of all, they are typically a lot less efficient than binary files, both in terms of storage size and reading efficiency. For example, I've seen ~10x improvements in size and performance when we were switching some large csv files to a binary format. 

Second, many text formats have syntax rules that a text editor might not understand. A text editor will happily store files containing errors, and you might only discover it later when trying to parse the file. If you've ever pushed something to CI that then reported 15 minutes later that you've made a trivial syntax error in some configuration file, that's what I'm talking about. 

Third, text files inherently mix content and presentation. Take programming languages as an example: They usually don't care where you put additional whitespace, but your choices are stored in the file anyway and shared with the whole team. There have been countless discussions about code formatting rules, preferable line lengths or whether or not to use curly braces for code blocks. There's no generic right answer to these questions because they are depending on individual preference. People have different tastes and needs, so there's no one-size-fits-all.

So what if all of these issues could be solved while still allowing the text editing we like so much?

My idea is to create a public registry of so-called "converter plugins". These plugins are [WASM](https://webassembly.org/) modules and basically provide two functions, **present** and **store**. **present** takes a sequence of bytes and converts it into a text. **store** does the opposite, taking a text and converting it into a sequence of bytes.

A text editor using these kinds of plugins works like this: When opening a file, the editor checks the file format and lets the user choose a compatible converter plugin. The plugin converts the file content into a textual representation. This text can then be edited just like in a regular text editor. When saving the file, the textual representation gets parsed and converted into the storage representation.

This simple enhancement provides a decoupling of how files are stored on disk and how they are displayed in a text editor. This has some big advantages:

- Editing binary file formats like CBOR, BSON, compressed text files etc. becomes possible without manual conversion.
- Syntax errors are caught early (when storing the file). Files containing syntax errors cannot be saved.
- Designers of file formats (e.g. for programming languages) can focus on the the relevant content and don't need to commit to specific syntax. This makes language evolution much simpler.
- Textual syntax of PLs would be part of a plugin. There could be different plugins for the same language to allow different tastes or simply experimentation with new ideas.
- There's more, so stay tuned for updates during the jam.

I've been experimenting with this idea over the last couple of months and have written some code already. I want to use the jam to bring these experiments into a usable state and focus on sharing the idea more broadly. 

Changing something as fundamental as text editors, it really feels like I'm reinventing the wheel here. Let's see if I can get rid of some of the larger edges our current wheels are having ;-)

I'm looking forward to the jam and especially to the other's projects. If you're working on a text editor and would like to integrate support for these plugins, I'd be glad to collaborate. See you soon!