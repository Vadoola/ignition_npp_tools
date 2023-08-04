This is a base working version of a Notepad++ plug-in written in rust. It's purpose is to allow quick shifting of an Ignition Perspective pipe by a set amount.

In Ignition Perspective using a Coordinate Container, there is no easy way to move pipes around precisely. Say I need shift my P&ID drawing up by 20 pixels, and to the right by 10. I can either try and drag it using the mouse and may not be precise, or I can go into the pipes properties, do the calculations and manually update the coordinates for each and every connection which is accurate but slow.

When you copy a pipe it just copies the raw json to the clipboard. This plug-in will shift all the coordinates by the given X and Y values, and then you can copy and paste the updated json back onto the pipe in Ignition Designer.

It's pretty basic, and just uses a basic regex find a replace for all x and y coordinate values.

As someone pointed out, I probably could have done this from within Ignition as a Python script...true, but I guess part of me wanted to
1. Practice some Rust / C FFI
2. See if I could create a Notepad++ plug-in.

Why choose [Slint](https://slint.dev/) for the dialog box? I initially tried using the Rust [Native Windows Gui](https://crates.io/crates/native-windows-gui) crate since it's just a single simple dialog box, but I have a hi DPI monitor and I couldn't get it to respect the scaling settings. Out of the other Rust Gui options, the only 2 I've spent any time with were Druid (work has moved to Xilem which isn't ready) and Slint, so that seemed like the quick and easy choice.

## Using
I've never used Github Actions, before, and as of right now I don't have it building the dll and uploading the artifact. Until I get this working you will need to build it yourself.

# Building
If you already have [rust](https://www.rust-lang.org/) installed (on Windows since that's the only official platform of Notepad++, although I've heard it works well in Wine), it should just be a matter of checking out the code and doing a cargo build --release

# Installing
Once you have built the dll create a folder titled "ignition_npp_tools" under the plugins directory for Notepad++ and drop the dll in there. Start Notepad++ and you should find it in the menu under Plugins->Ignition Tools->Move Pipes
