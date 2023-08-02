This is a base working version of a Notepad++ plug-in written in rust. It's purpose is to allow quick shifting of an Ignition Perspective pipe by a set amount.

In Ignition Perspective using a Coordinate Container, there is no easy way to move pipes around precisely. Say I need shift my P&ID drawing up by 20 pixels, and to the right by 10. I can either try and drag it using the mouse and may not be precise, or I can go into the pipes properties, do the calculations and manually update the coordinates for each and every connection which is accurate but slow.

When you copy a pipe it just copies the raw json to the clipboard. This plug-in will shift all the coordinates by the given X and Y values, and then you can copy and paste the updated json back onto the pipe in Ignition Designer.

It's pretty basic, and just uses a basic regex find a replace for all x and y coordinate values.