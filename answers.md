Title: Problem Set 1 Answers
Author: Matt Pearson-Beck

Problem 1.

User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/32.0.1700.77 Safari/537.36

I analyze this piece by piece below:

* Mozilla/5.0: I read some really interesting history about browsers and discovered that initially, Mozilla was served frames while other browsers weren't (based on the user-agent string), so other browsers (IE at first) just started pretending to be Mozilla when they began supporting frames to avoid being "approved" for frames. The 5.0 was a version that Mozilla started using in 2002.

* X11: This is the one I had the hardest time figuring out. From what I can tell, it refers to a variant of X-Windows, which is a windowing system for bitmap displays (according to Wikipedia).

* Linux x86_64: I'm running a 64-bit version of Linux.

* AppleWebKit/537.36: Chrome uses WebKit, which is something Apple made when developing Safari.

* KHTML, like Gecko: Just like with "Mozilla", some good web code was only given to browsers using Gecko as a rendering engine. Linux uses Konqueror but was not given this good web code, so they pretended to be Gecko to get good code by including the "KHTML, like Gecko". KHTML stands for Konqueror.

* Chrome/32.0.1700.77: This is the version of Chrome that I'm currently running (checked in settings).

* Safari/537.36: Chrome also impersonates Safari (I'm not really sure why).

One cool passage I found on that site (http://webaim.org/blog/user-agent-string-history/) is: "And then Google built Chrome, and Chrome used Webkit, and it was like Safari, and wanted pages built for Safari, and so pretended to be Safari. And thus Chrome used WebKit, and pretended to be Safari, and WebKit pretended to be KHTML, and KHTML pretended to be Gecko, and all browsers pretended to be Mozilla ... and the user agent string was a complete mess, and near useless, and everybody pretended to be everyone else, and confusion abounded."

-------------------------------------------------------------------------------

Problem 2.

Rust places a huge focus on catching errors at compile time, which means it wants to know as much as it possibly can about what state the program will be in at certain points in the code. Global variables make it almost impossible to tell what the state of the program will be at any given point that they are used, because they could have been changed anywhere else in the program at any time. I believe this is why they are considered unsafe, but why we still *can* use them (with the unsafe blocks).

In terms of tracking the state the program will be in when global variables are used, my inclination is that it would be just as hard as the Halting Problem (so undecidable), which is why the compiler can't do it.