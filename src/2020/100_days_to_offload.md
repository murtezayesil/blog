# \#100DaysToOffload

I’m publishing this as part of 100 Days To Offload. You can join in yourself by visiting https://100daystooffload.com  

Legend says that it is better to write 100 posts for 100 consecutive days. But these are guidelines, not strict rules. If they were, I wouldn't even start. Instead I will update this post regularly. I am not writing 100 separate posts because it feels like spamming my own blog.  

**[Inhale]**  

...  

**[Exhale]**  

Here we go

---

##### 1

1. Yay 😃️ I finally took on the challenge  

2. Today I went to GitHub to delete many repositories I forked and abandoned over the years. One of them was my i3wm config repository. I don't worry about losing my i3 config really. What I wanted to salvage from it was my tty-clock like wallpaper generator script.
   
   Old script didn't use variables and was hard to modify. I started rewriting it today. It was a challenge to write it back in the day and it is a challenge to rewrite it again. I am learning many things about bash while trying though. Bash scripting can be mind tickling. This is why it is fun.

3. I purchased hosting on Digital Ocean and deployed my OwnCloud instance. Some people wonder why I choosed OwnCloud over NextCloud. OwnCloud makes better pun.

---

##### 2

1. I am done with desktop-clock script. Get it on [github](https://github.com/murtezayesil/student/tree/master/sh/desktop-clock)
   
   ![](./desktop-clock.png)
   
   Note: I tested the script on 1366x768 display and defaults are good for such displays. Remember to change values if you have a monitor of different size or store wallpapers in somewhere else than default. 

2. I thought it would be better to send emails for my family members for them to set up their account on our shared OwnCloud instance. I tried Postfix since I  don't need a feature rich email server. But configuration is too complicated for me. Using a sub-domain instead of main-domain really puts me in a difficult position.

---

##### 3 - Fizz

1. I wanted to implement send-only email server to my OwnCloud instance [yesterday](#2). But it turs out, SMTP and Postfix stuff is way over my head. I will postpone it to later time.
   
   Not to mention, I am scared to break currently working system. I should try new things in a burner (testing) server first. And only after a succesfull prototype I should go ahead and do changes in main server.
   
   You probably knew that testing first would be a better idea. Guess what! I am a newbie who is just coming to these realizations.

2. After a long time away from Java, I tried a simple program to see if I can still right something. I wrote a program that validates [ISBM-10](https://github.com/murtezayesil/student/tree/master/java/ISBNcheck/src/com/company) code.

---

##### 4

I started to Computer Science degree on [OSSU](https://github.com/ossu/computer-science). You can start to CS degree during lockdown. All you have to the is to [follow this curriculum](https://ossu.firebaseapp.com/#/curriculum).

---

##### 5 - Buzz

I decided to [Indieweb](https://indieweb.org/)[ify](https://indiewebify.me/) myself. But I am currently using [mdBook](https://rust-lang.github.io/mdBook/) to generate static online book from my posts to use my blog. This produces an amazingly fast and clean website. But it can be difficult to implement anything from outside. There are no plugins for mdBook like there are for bloging and CMS focused systems.  

It is a difficult to chose a blogging system/platform when you have [over 100 choices](https://alternativeto.net/category/social/blogging/). Thankfully I have FOSS and static pages as my constrains to filter them down to about 20. I really am seeing the benefits of saying [no](https://www.williamury.com/books/the-power-of-a-positive-no/).

After some more research, I am inclined towards [Known](https://withknown.com), thanks to its indieweb integration.

---

##### 6

1. Still comparing [blogging solutions](#5---buzz). I have a new constrain to help me in my desicion. My blog will be static. I want it to run on [GitHub Pages](https://pages.github.com/) due to financial constrains. GitHub Pages is amazing for hosting a static website and costs nothing.
   
   But I don't mind using PHP powered CMS to generate static pages and then upload static pages to hosting.

2. I wanted a landing page for a long time. In fact, having a landing page was driving force behind me purchasing this very domain, murtezayesil.me. Today, I started to do some testing and for the landing page and I forked [this template](https://github.com/flexdinesh/dev-landing-page). You can see my current progress [here](../landing-page/candidate1/index.html "Landing Page - Candidate 1").

---

##### 7

1. Wrote [Hello, Linux!](./2020/May/hello_linux.md)

2. Today I noticed that mdbook only generates HTML pages from pages linked in SUMMARY.md (in sidebar). So, when I move a link form sidebar to archive, that page will be removed form blog 😞️
   
   This will force me to move away from mdbook. Mdbook was not meant to be a blog generator anyway.

---
