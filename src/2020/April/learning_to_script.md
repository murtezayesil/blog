# Learning to (Shell) Script

This is an excessively long story of me writing a [one-liner](https://en.wikipedia.org/wiki/One-liner_program).

Recently [a friend](https://radical.town/@olivia) who was editing Polybar config to complete her BSPWM setup needed a way to visualize the hex-colors in config file. She did use some extension on Atom or VScode to visualize hex-colors but, according what she said, text editor got slower. That is understable since extension probably was parsing the entire file every second to find hex-color codes. She decided to toot about her need of finding an alternative way to visualize hex-colors.

I saw her toot and thought to myself "This can be a great opportunity to learn to script.". Since I used i3wm, feh and imagemagick in the past, I knew I could even write a script that could turn her wallpaper into the color she highlighted. But that is for maybe another day.

First thing is to find the color from hex-color code. Next is to turn it into an image.

1. Nice thing is that Linux has 2 clipboards:
   
   - Primary clipboard is cursor driven. It **copies by highlighting** anything and **pastes by clicking with MMB** (Middle Mouse Button).
   
   - Second clipboards is short-cut driven. It copies highlighted after pressing Ctrl+C and pastes after pressing Ctrl+V

Luckily for me `xclip` command works with primary clipboard. Reading highlighted text is simple as `xclip -out` or `xclip -o` . I owe this knowledge to [this answer on Stack Exchange](https://stackoverflow.com/questions/749544/pipe-to-from-the-clipboard-in-bash-script).

Next is to find a way to show the color in clipboard. I found [this answer on Stack Exchange](https://unix.stackexchange.com/questions/482755/is-there-a-command-to-display-colors-when-giving-hex-value-in-terminal). Now I also know how to visualize. We can set xlogo's background color via `xlogo -bg $color` command where `$color` will be the \#hex-color code.

I learned some shell stuff from books I purchased from O'Reilly. I knew that I could input a standard output of a command as argument of another command by passing the command as argument wrapped in \`\` (DO NOT confuse (\`) with single quote (') character). This helped me to replace `$color` with `xclip -o` to make a one-liner.

```shell
# My script
xlogo -bg `xclip -o`
```

Meanwhile, she was making her research and read about a command called `zenity`. I didn't know about `zenity` command either. It turns out, it is a GTK dialog generator. It can display a calendar, show error pop-up, question prompt and even show color-wheel. This would allow her to see the color and even change it using the color-wheel. She was able to construct her own script with what she learned from my script.

```shell
# Olivia's script
zenity --color-selection --color=`xclip -o`
```

This was a great experience. I got the opprtunity to make research on the internet and read man pages which I should get used to.

This may seem novice to long time terminal users and scripters. But for me, this experience felt like a growing pain or my first steps. This is not the first time I wrote a script. But that is my first time coming up with a one-liner that is short, brief and just works.

```shell
# I modified my script a little bit to show meaningful
#  error when no color is highlighted
color=`xclip -o` && xlogo -fg $color -bg $color
# This will only show the highlighted color OR an X :D
```

>  Hex-color
> 
> > RGB (Red Green Blue) or RGBA (RGB Alpha) color code that is written in Hex (base 16) number system. Commonly used to define colors in webpages.
> 
> > \#FF0000 = Red
> > 
> > \#008888 = Dark Turquise
