# don't_fear.tar

###### This is my story with LFS, not a tutorial.

_LFS_ (Linux From Scratch) is one of the weirdest project that I wanted to tackle. Not because I needed something more Arch than Arch Linux [^1] but it requires a deeper understanding of Linux System and Shell to build and install, which I believe is curicial for my future SysAdmin career.

[LFS Book](http://www.linuxfromscratch.org/lfs/view/stable/ "Current Stable LFS Building Guide") is more of a guide than instructions manual. It doesn't only give you shell commands with bunch of arguments but also describe what each each arguments mean and how to use them. In my opinion, it is the man pages for dummies. 

LFS uses tarballs, .tar files, to distribute source code of each software. It is not unique to LFS. Tarballs are used everywhere in Linux world and I believed I needed to get myself confortable with dealing with tarballs.

## What are tarballs?

A tarball (or tarbomb) is nothing but a [.tar](https://en.wikipedia.org/wiki/Tar_(computing) "Read on Wikipedia") file. It is called tarball because it causes an explosion of files wherever it is unpacked.  

Name comes from "Tape ARchieve" because it has no indexing to allow random access to file. Reading a file in a tarball means also reading everything until that file, just like a tape. It is not suitable for reading individual files in a large archieve.  

It has no compression capability but everything in a tarball can be easily compressed by compressing tarball itself. Tarball will have .tar.gz extension if it is compressed with gzip algorithm for instance.

## Why use tarballs?

They are like suitcases 💼️. We can put so many tiny files 📄️ into 1 file to make distribution, carrying, compression, and archieving easier. GitHub produces a tarball and a zip file of the git repository with every release.

Plain text files can be compressed further to take extremely small space while carrying so much data or code. I recommend tar.gz over zip every day of the week.

# Why build Linux From Scratch ?

I wanted to have much better understanding of Linux file structure and know where everyhting (headers, libraries, binaries, configurations, device files etc.) goes. I believed that would be important since I wanted to become a SysAdmin. I ended up scratching the surface.

## Short Story

I initially used my old Sony VAIO laptop to build a 32bit LFS system. I was handicapped with current 32bit support. I tried my luck on my 64bit Lenovo laptop. There were no headaches until I wasn't able to boot due to UEFI. I probably could if I compiled more of the dependencies for UEFI support for GRUB. But since my main goal was learning 

## Long Story

I thought it would be a good opportunity to make use of Sony VAIO laptop from 2007. Its 1.7GHz Dual core CPU was bottlenecked by my knowledge at the time. Stunning 1GB DDR2 RAM was complimented with light DE of Manjaro. After all, I needed every MB I could get since I had a browser tab open for reading the [LFS Book](http://www.linuxfromscratch.org/lfs/view/stable/ "Current Stable LFS Building Guide"). I would also share my progress on fosstodon after compiler finished its job. It didn't result to a working system because Manjaro and Arch dropped support for 32bit CPU and community packages were lacking some header files I guess that compiler needed for 32bit build.  
I later decided to try LFS again on my Lenovo IdeaPad 110. 2.2GHz Quad-core CPU + 6GB RAM felt like an upgrade from a bicycle to car. I compiled a 64bit system with no support for 32bit software. I only wanted to run neofetch on it to show that I had a running LFS system. UEFI boot was an issue that I never got around. But I also lost interest at that point since my main goal was satisfied, learning more ins and outs of a Linux system.

### What is next?

I want to build backup and file sync servers as my next project in my journey to become a self-taught SysAdmin. I really want to utilize same old Sony VAIO laptop. I am not rushing though. I am abroad for my study and I want to start to it when I get together with my family. But for now, I am learning Rust programming.

I may upgrade few things in Sony when I start my sysadmin related projects. Mostly to prevent headache that could be caused by very old hardware. I don't want HDD to die right after I build a working server for example.

[^1] Linux geeks will understand
