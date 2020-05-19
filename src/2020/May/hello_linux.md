# Hello, Linux!

I previously wrote about [my attempt to LFS](../April/dont_fear.tar.md). But I didn't start using Linux after compiling the kernel, oh no. I meet Linux when I was 9th grade years old, around 2010 I believe. As a member of Project Club, I had opportunity to go to Ankara to visit Science Fair by TÜBİTAK, Scientific & Technological Research Council of Turkey. I purchased an issue of the Science Technic magazine but I was more drown to CD that came with it. It was an installer for [Pardus](https://distrowatch.com/table.php?distribution=pardus) distro.

Pardus shipped with KDE at the time and it was too heavy for our family laptop. 1.6GHz single core CPU was bottlenecked by 512MiB RAM. KDE wasn't very optimized and it was a laptop, QT didn't benefit from any kind of GPU acceleration. But Pardus had one thing that made us stick to it till that laptop never boot. WiFi and Speaker support out of the box 🎉️ Remember, only other OS I used and knew was Windows XP. And its "out of the box" support was limited to CD reader, USB mouse/keyboard. XP couldn't even play audio without a driver. That laptop lived till mid 2012 and it was \~7 years old when it died.

Fast forward to 2017, I needed a laptop for practicing C Language at hostel. I felt it was necessary to make the purchase because Turbo C++ compiler on DosBox was cumbersome at best and straight out broken at times. I settled for Lenovo Ideapad 110 which I am using right now. It has 4 CPU cores at 2.2GHz and integrated GPU for \$400. It didn't have a preinstalled OS. I had to install an OS that I could afford. Because I knew about Linux, I tried Ubuntu. Graphical installer was easy to use and every single thing on the device (including microphone and bluetooth) worked flawlessly out of the box. I didn't need to open a single terminal to install the OS. After OS installation, I installed an IDE for C coding. Everything needed was installed along with it. The best thing is I wasn't hunting some executable in the internet. Today I know that every program Linux users install are tested by the developers of the OS they are using. Using Software Center/Package manager meant I was only getting software from trusted first parties, not from shady websites. This is one of the many reasons Linux is safer.

> I don't know why people say Linux is difficult. Everthing worked for me out of the box.

> Installing programs from Software Center means you are only installing from trusted developers. Not from shady corners of the internet.

After some time, I got an itch. I was feeling adventurous and wanted re-install everything with better partitioning. I learned that using entire 1TiB of HDD as root directory (annoted as / [slash] ) was not the best way to organize data. I wanted to make more partitions and try other distro options.

Next I installed Kubuntu. I didn't like KDE and uninstalled kubuntu-... package. I uninstalled something called Xorg in the process. As it turns out, Xorg is the program that uses GPU to provide a GUI, so that I can use browser and IDE.

Next, I installed Linux Mint 17. It was beautiful. Worked OK but I couldn't stick to it for some reason. I wanted to try something outside the Debian family.

I went Arch. This was like and overdose of Linux for me 😵️ It lived for about a week or so. I broke my system very fast. I can't blame Arch. I was amateur.

PopOS announced that they were gonna ship their own OS. I was annoyed with needing to do major updates every 6 months. After considering between Solus and Manjaro I went Solus. You can guess why not Manjaro 😜️

Solus (3.999) had 2 issues. After every update system would boot to Black Screen instead of Login screen. I had to manually boot into LTS kernel and reboot. Solus is a Rolling Release distro. In other words, I would have black screen issue multiple times a week. ... Then steam broke. This was the last drop. I hopped to Manjaro.

Manjaro is like stable Arch for an amateur like me. It was amazing and I used it for months. But Valve Proton support wasn't as good as on Ubuntu side.

Later I moved to Elementary OS. It is beautiful. Not only on appearance but in system resource usage too. It was freaking fast and light compared to my previous experiences with Linux Mint and Manjaro. I later heard that Linux Mint 19 offered much better optimized OS. I hopped.

About 2 months ago I switched to Linux Mint 19. Elementary is still installed along side it. But I never boot Elementary. Don't need to. I have my Rust coding environment in Mint. Why would I want to hop...

> You are all caught up

Wait a minute. Solus Budgie seems to be working amazingly well for other people. I may install it in place of Elementary and ....

You see the cycle? I don't distro hop because I need to, at least not all the time, but because it is an adventure.  

---

After trying many OSes, I decided to make an opinion based table according to their out-of-the-box state. Of course every distro offers an amaing level of customizability! So, there is no perfect distro. They are all amazing.

| Scored / 5              | Aesthetics | Familiarity<br/>to Windows switchers | Simplicity | System Resource<br/>Usage |
| -----------------------:|:----------:|:------------------------------------:|:----------:|:-------------------------:|
| Elementary<br/>5.1      | 5          | 1                                    | 5          | 5                         |
| Manjaro<br/>KDE (2019)  | 3          | 5                                    | 3          | 4                         |
| Mint<br/>Cinnamon 19    | 4          | 5                                    | 4          | 4                         |
| Solus<br/>Budgie (2018) | 4          | 4                                    | 4          | 3                         |
| Ubuntu<br/>GNOME 18.04  | 5          | 1                                    | 3          | 2                         |


