NOTE: Under construction! 

This is a rewrite of my earlier `chiropterm` library, which had some serious API design issues:

- The system for windowing and clipping the draw cursor was verbose and bureaucratic.
- The callback-based APIs made state management complicated. 
- Forcing a total redraw every frame was un-Curses-like

Compared to `chiropterm` a lot of things have been simplified -- there's no automatic resizing, no color management. The input subsystem actually got more complicated in an attempt to support both GameMaker-style "query for what buttons are down" input and event loop-based input. 

Probably don't use this yet -- it's a serious WIP!