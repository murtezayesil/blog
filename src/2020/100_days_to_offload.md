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
   
   Note: I tested the script on 1366x768 display and defaults are good for such displays. Remember to change values if you have 

2. I tried Postfix for send-only email server. I didn't want to install a more feature rich email server for only sending announcement and password reset emails. But configuration is too complicated for me. Not to mention, using sub-domain instead of main-domain doesn't help to configure.

---

##### Fizz

1. I wanted to implement send-only email server to my OwnCloud instance [yesterday](#2). But it turs out, SMTP and Postfix stuff is way over my head. I will postpone it to later time.
   
   Not to mention, I am scared to break currently working system. I should try new things in a burner (testing) server first. And only after a succesfull prototype I should go ahead and do changes in main server.
   
   You probably knew that testing first would be a better idea. Guess what! I am a newbie who is just coming to these realizations.
2. After a long time away from Java, I tried a simple program to see if I can still right something. I wrote a program that validates [ISBM-10](https://github.com/murtezayesil/student/tree/master/java/ISBNcheck/src/com/company) code.
