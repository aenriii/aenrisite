---
title: writing a dynamic site without clientside code
description: a nearly tall tale of svelte, rust, and probably like, 200+ hours

date: 2024-11-23

---

# writing a dynamic site without clientside code
-- a nearly tall tale of svelte, rust, and probably like, 200+ hours

here told is the terrible, terrible story of me trying to make a website
where i would be satisfied with the server- and client-side performance,
style, and general ease of code maintenance.

this tragic tale pushed me beyond 200 hours of development on a simple
website that shouldve taken maybe 10.

## where it all started
-- honestly everything about this was a learning experience

### first attempts
-- a shitty site in client-side svelte

as of writing this, [aenri.loveh.art](https://aenri.loveh.art) still holds
the v1 of my website. this was an amateur attempt at learning svelte, and
it resulted in a website where you could not change the active tab without
javascript completely enabled. in addition, over time, i began to dislike
the bubbly catppuccin aesthetic when used to represent myself. that, and
a lack of code quality, features, and maintainability, led me to decide
that the best thing to do would be to start over.

### if at first you dont succeed, server-side render it
-- css can be a powerful programming language

did you know that you can use pseudoselectors and various other filters to
only show elements if a radio element is selected? i found that out this
time around, using an assortment of what i liked to call "css fuckery" to
remove the javascript barrier to switching tabs. within this iteration, a
shift towards a more straightforward design was seen, with rounded fonts
often being traded out in favor of bold header fonts and plain but strong
text and subtext fonts. this iteration also marked the beginning of my test
of spite, against both my past self and those who claimed against my ideas;
i was going to make a dynamic site with as little client-side js as possible,
using the server as my renderer and my content-loader.

#### anti-js means a server-side native module, of course

here's where things start to get performance-at-any-cost. i wanted to include
a feature on my website to sync my spotify now-playing (as i used spotify
at the time) to animated lyrics on my website (based on the design of apple
music's lyrics). to get the information i needed to do this, i needed a few
simple things:

1. [size=1.5em] the ability to get my now playing status

simple enough, just make an application on the spoitfy dev portal and put the
pieces together with some oauth here and a couple scopes there.

2. [size=1.5em] the ability to get lyrics from a spotify song id.

this proved to be the most brutal part, at least for as far as i got. i
attempted scraping the api of a spicetify plugin, i tried using respot,
i tried just about everything i could think of including re-implementing
a smaller bit of librespot

3. [size=1.5em] a server-side method for quick html/css codegen to animate lyrics

oh dear god.


