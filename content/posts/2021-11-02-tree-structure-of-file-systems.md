+++
# layout = post
title = "The Tree Structure of File Systems"
description = "For some data structures typically represented using files and folders, another tree structure would be simpler and more intuitive."
# date= 2021-11-02 19:55:00 +0200
# categories = rust file system tree structure folder directory 
+++

I've been using file system for a long time and have always thought of them as tree data structures. A couple of days ago, I had a realization that seems obvious in retrospect, but didn't occur to me all those years before: The file system tree is different from what I'd usually implement as a tree structure.

If you'd ask me to implement a tree data structure, I'd probably write something like this:

*(I'm using Rust throughout this post, but you should be able to follow along without prior knowledge of Rust.)*

```rust
struct SimpleTreeNode<T> {
    data: T,
    children: Vec<SimpleTreeNode<T>>,
}
```

The tree consists of nodes, which I've called `SimpleTreeNode`. Each node is a struct containing some data and a list of children. Because `children` contains other nodes, this structure can represent arbitrary tree-like hierarchies. The `data` attribute isn't strictly necessary to create a tree, but will be needed in almost all cases. You typically want to associate some data with each node in the tree.

If we try to create a simplified version of the tree data structure that file systems offer, we'll end up with something different:

```rust
enum FileSystemNode {
    Directory {
        name: String,
        children: Vec<FileSystemNode>,
    },
    File {
        name: String,
        data: Vec<u8>
    },
}
```

This time, our `FileSystemNode` is an enum instead of a struct. It contains two different cases, one for directories and one for files. Both of them contain a `name` attribute, which is metadata. Additionally, the `Directory` case also contains an attribute `children` which contains a list of child nodes. This is equivalent to the previous example and allows representing the tree structure. The other enum case, `File`, contains actual `data` but no children.

What we can see from this definition is that **actual data can only be stored in leaf nodes**. Data can only be stored in files, and those can't have children. On the other hand, nodes that have children (directories) cannot store data.

We can see that very practically using standard linux / programming tools. While you can use `cat` to show the data of a file, it won't work for directories. And the other way around, you can't use `cd` to navigate into a file or `ls` to list its children. Another example can be found in your IDE: Double-clicking on a folder doesn't show content in the main editor area and you can't create sub-files of a file through a context menu entry.

You may now be thinking "Why would I want to do this?". If you're thinking of files and folders using their actual analog equivalents (sheets of paper in a folder), opening a sheet of paper (`cd` equivalent) doesn't make any sense. On the other hand, you could probably argue that paper folders can actually contain data themselves if you write it on the cover, but that's not a solution for larger amounts of data. 

But we're not talking about analog paper based data organization, we're talking about computer file systems. While the file, folder and file system analogies made it easier for people to transform their data organization from analog to digital, it's probably time to move on by now. I was born in the nineties and haven't really experienced that analog world myself. For me, sorting a bunch of files means clicking on the top of a column in the file explorer instead of sorting paper by hand.

Anyway, what I'm trying to say is that our computer file systems are inspired by analog file systems and that this probably doesn't make a lot of sense (anymore?).

The fact that inner nodes in a file system hierarchy cannot hold data is limiting. In many use cases, it'd be natural to have nodes that can hold data AND have children. Because file systems don't allow this, we often use workarounds. Let's take a look at some examples:

## Rust modules

*Giving a full explanation of how modules work in Rust would be beyond the scope of this text. If you want to learn more about them, I can recommend the official [Rust book](https://doc.rust-lang.org/stable/book/). [Chapter 7](https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) explains [what modules are](https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) and [how they can be separated into different files](https://doc.rust-lang.org/stable/book/ch07-05-separating-modules-into-different-files.html).*

In Rust, the root of your [crate](https://doc.rust-lang.org/stable/book/ch07-01-packages-and-crates.html#packages-and-crates) is usually `src/lib.rs` (for libraries) or `src/main.rs` (for binaries). Within this root file, you can create modules using `mod` blocks. Modules can be nested and their definition can either be inline or in separate files. When using separate files, there's rules for naming them.

Let's say we have the following hierarchy of modules, implemented as inline modules in the root file:

```rust
// in src/main.rs
mod foo {
    // some content
    mod bar {
        // some more content
    }
}
```

There's a module `foo` that contains another module `bar` and both of them might contain more code. If we want to split these modules into separate files, we can do it like this:

```
src/
  main.rs     # main module
  foo.rs      # foo module
  foo/
    bar.rs    # foo::bar module
```

First level modules (`foo` in our example) can be put into the same folder as the root file (`main.rs`) and are named after the module name (`foo.rs`). Nested modules (`bar` in our example) need to be put into subfolders that are named after the parent modules. The path of module `bar` therefore is `foo/bar.rs`.

The content of the different files needs to be adapted as well:

```rust
// in src/main.rs
mod foo;
```

```rust
// in src/foo.rs
mod bar;

// some content
```

```rust
// in src/foo/bar.rs

// some more content
```

When pulling inline modules into separate files, the module definition is changed from `mod foo {...}` into `mod foo;` in the parent module. The contents of the block is moved into the newly created module file (`foo.rs`). The same change is done to the nested module `bar` as well.

Rust also allows using `src/foo/mod.rs` instead of `src/foo.rs`. The content of all files can stay the same, but the structure would then look like this:

```
src/
  main.rs     # main module
  foo/
    bar.rs    # foo::bar module
    mod.rs    # foo module
```

Are you confused? At least I was a bit confused when I first tried to learn Rust in 2018 and I've heard from others who were having trouble learning how modules work in Rust as well. Once you understand it though, it makes sense and the only thing left to remember is where to put module files and how to name them properly. Also, when reading other's code, there's always two places to look for a module file. Is that necessary?

Modules in Rust can have sub-modules AND contain code themselves. This is exactly the case I've talked about earlier: Wanting a node that both has children and data itself. File systems cannot directly represent this, so Rust's workaround is to have a folder containing child modules (`src/foo`) AND a special file either next to (`src/foo.rs`) or within (`src/foo/mod.rs`) it.

**What if we would have files with children (or directories with data respectively)?** The example structure above could look like this:

```
src/
  main.rs     # main module
  foo.rs      # foo module
    bar.rs    # foo::bar module
```

In this hypothetical file system, the `foo.rs` file would be kind of both a directory and a file. Using `cat` on it would display the module's content and `ls` / `cd` could be used to query its children.

That looks pretty cool and pretty useful. Not only have we eliminated one file system element, you also don't have to think about whether to put the special file (next to or within the folder?). That's a good hint that this hypothetical file system's structure naturally matches the structure of our use case.

## Html Websites

I've chosen Rust as an example above, but the same issue (and the same workarounds) can be seen in many other areas as well. For example, a simple html website might be structured like this:

```
blog/
  posts/
    index.html
    hello-world.html
    file-system-tree.html
  index.html
  about.html
```

The website consists of several html files in nested folders. Serving the `blog` folder using a simple HTTP Server allows it to be viewed using a web browser. For example, if the server is running locally on port 8000, pointing a browser to `http://localhost:8000/blog/about.html` would display the content of the `about.html` file. Similarly, using `http://localhost:8000/blog/posts/hello-world.html` would show the respective hello world file. So far so good.

But what happens when we're pointing the browser to a folder instead of a file (e.g. `http://localhost:8000/blog/posts`)? As always, [Wikipedia has the answer](https://en.wikipedia.org/wiki/Webserver_directory_index):

> When an HTTP client (generally a web browser) requests a URL that points to a directory structure instead of an actual web page within the directory structure, the web server will generally serve a default page, which is often referred to as a main or "index" page.
>
> A common filename for such a page is `index.html`, but most modern HTTP servers offer a configurable list of filenames that the server can use as an index.

In our case, requesting `http://localhost:8000/blog/posts` will show the `index.html` file in the `posts` folder. The `index.html` file contains the data that logically belongs to its parent folder (`posts` in this case).

That's very similar to the Rust example, except that the "special directory file" is named `index.html` instead of `mod.rs`. And just like in the Rust example, we could simplify the structure when allowing files that can have both data and children:

```
blog.html
  posts.html
    hello-world.html
    file-system-tree.html
  about.html
```

In this example, the data that has been stored in `blog/index.html` and `blog/posts/index.html` respectively is now directly stored in the parent nodes (which I've called `blog.html` and `blog.html/posts.html` respectively).

Again, this change reduced the number of file system elements and there's no more need for the "special directory files". Looking at the quote above, it would probably also remove a configuration option from http servers. The data that should be served for a directory would be clear: it's the data stored within the directory's node.

## Saving a Web Page

Have you ever saved a web page from your browser? Simply point your browser to some site (e.g. `https://www.google.com`), right-click somewhere and select "Save page as..." from the menu. This will show that an html file is being downloaded, but that's not entirely true. Besides the html file of the page, the browser will also create a folder containing all other files used by the website (e.g. pictures). In the case of google, I get these files / folders:

```
Google_files/
  ...                // lots of files (pictures, js code, ...)
Google.html          // main html file
```

That's similar to Rust's other approach: Using a file next to the folder containing child elements (`src/foo.rs`). The hypothetical file system could allow this structure instead:

```
Google.html          // main html file
  ...                // lots of files (pictures, js code, ...)
```

There's many more examples like these. Once you start looking for them, you'll see them everywhere. 


## Another File System Tree

If our status quo is limiting and requiring workarounds for relatively common cases, we should probably be thinking about alternatives. What if we were changing the structure of a file system from 

```rust
enum FileSystemNode {
    Directory {
        name: String,
        children: Vec<FileSystemNode>,
    },
    File {
        name: String,
        data: Vec<u8>
    },
}
```

into

```rust
struct FileSystemNode {
    name: String,
    children: Vec<FileSystemNode>,
    content: Vec<u8>,
}
```

At first sight, the second definition is simpler. There's no separation between files and folders anymore, just nodes. It's also more general as these nodes can have both children and data.

At the beginning of this text, I showed a simple (if not the simplest) tree data structure:

```rust
struct SimpleTreeNode<T> {
    data: T,
    children: Vec<SimpleTreeNode<T>>,
}
```

When looking closely, we can see that we can use `SimpleTreeNode` such that it contains the same data and structure as `FileSystemNode`.

```rust
type FileSystemNode2 = SimpleTreeNode<(String, Vec<u8>)>;
```

Here, we're using a tuple of a string and a vector of bytes as the generic parameter `T` of `SimpleTreeNode`. The string can hold the name of the node and the byte vector the data. Accessing the attributes will look a bit different, but it's basically the same data structure.

Fascinating, isn't it? By removing the limitation we've seen before and simplifying the data structure, we ended up with what's one of the most fundamental data structures in computer science. That feels good!

--

The tree structure implemented in state-of-the-art file systems is limiting and requires workarounds for common use cases. It's possible to change the structure to make it both simpler and more general.

If I would be designing a file system from scratch, I would seriously consider using this simple and general node-based structure and drop the separation between files and folders. It's fascinating to think about how programs, dev tools and even operating systems could be designed differently when based on this kind of structure. 

As software developers, we are used to building layers upon layers. The further down the stack we can achieve improvements, the larger the positive impact can be. File systems are a fundamental part of all operating systems and pretty low down that stack. It's important to try to rethink and improve these fundamental building blocks. I hope that the thoughts outlined above will be a small contribution towards that goal.
