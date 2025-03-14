
# Fantasy Tavern Maker
***For Fantasy Worlds Settings such as Dungeons & Dragons 5th Edition***

## Current Version
0.5.1

## Purpose
Most Games Masters or Authors are aware that they need to unique and
interesting Pubs, Taverns, Inns, Wayhouses and the like as part of the
adventure-travel narrative. However, it can be awkward to do so easily
on-the-fly without complex table rolling that interrupts the flow of the story.

This application is a RUST re-write of an original C# implementation
that I wrote which I was never happy with. So far, this version is much
more what I was hoping to achieve.

## Sample Output

```md
-----                        Player Blurb                        -----

 'The Checkered Hen' is the local Pub and Bed House for travellers in this area.
The modest-quality establishment would be considered large, with 25 tables.
It has 18 tent beds in the common room and 8 private rooms. Rooms are 5sp per day,
and meals are 3sp per day.

  As you enter, the air is full of the scents of fresh linen and hot spiced cider.
The current patrons seem to be a merchant-friendly bunch, and the main area is dimly lit by candles.
A sign hung from the fireplace mantle says 'We don't serve tieflings'.

 The menu has the usual standard fare posted.
The House specialty beverage is a locally made Clearfire, for 19 copper,
while the House specialty dish is stuffed and bacon wrapped serpent steak
 served with root vegtables, for 20 copper.


 -----                          DM Notes                          -----
 Establishment History:
 * The Checkered Hen is a multi-generation business, in operation for more than 25 years.
 * The establishment and its grounds look to be nearly brand new.
 * An infamous murder happened here.

 Red Light Services:  |Gambling (DC 12)|

```

# libraries in use

https://rust-random.github.io/book/quick-start.html -- RAND functions

https://docs.rs/cursive/0.21.1/cursive/ -- cursive
