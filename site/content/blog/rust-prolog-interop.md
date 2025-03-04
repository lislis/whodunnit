+++
author = "lislis"
title = "Rust-Prolog interop"
date = "2025-03-04"
description = "A brief outline of the process rabbit hole"
tags = [
    "rust",
    "prolog",
    "processes",
]
+++

One of the fundamental steps in the entire project is to be able for the Rust programm to interact with the Prolog interpreter.

Rust offers `std::process::Command` to call out to external programs, which seemed like a fair enough start. (This would mean that the Prolog interpreter would need to be installed seperately by the user. But I decided to leave this up to final stage packaging for future me :tm:)

While just calling out to Prolog ([scryer-prolog](https://github.com/mthom/scryer-prolog) in this case, of course it's a Rust based implementation) worked just fine. The base example however led to just starting the interpreter's own CLI interface inside my Rust interface.

Technically, this is not a problem, since I want to work within a session anyway. I do however need to have some game logic in Rust running in between the user and Prolog interactions.

For this I need to figure out how to "just" run the interpreter in a seperate background process and have Rust communicate between that and the user's input from stdin.
