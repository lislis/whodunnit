+++
uthor = "lislis"
title = "Rust glue code"
date = "2025-03-14"
description = "How to throw everything together in Rust and hope it works"
tags = [
    "rust"
]
+++

Originally I envisioned a really fancy terminal UI using [ratatui](https://ratatui.rs/) but looking at the ["Basic App" example](https://ratatui.rs/tutorials/counter-app/basic-app/) it became clear really quickly that this is completely out of scope. Too bad, it looks so good!

Another idea I had was to use parts of [bevy's](https://bevyengine.org/) entity-components-system to oragnize the game state. But then I realised that this is total overkill for what I'd have in the first iteration of this project.

So vanilla, unstructured Rust it is!

I started with the `PLHandler` struct to encapsulate the inital interaction between Rust and Prolog that I managed, including prompting the user and writing Prolog output to a file.

Then I added everything into a `Game` stuct that represents the entire game, state, progress and the talking to Prolog. While progress here was reasonably fast, I quickly realised that I alredy duplicated the work of user prompting and evaluating user inputs. Not good!

Ideally I'd refactor this part of the code into `generation`, `prompting`, `parsing`, `prolog`, (state) `management` modules. But you only know what makes sense once you have a gigatic bowl of spaghetti code to understand the patterns. At least that how it works for me.

In the end I have an lots of working early stage code with lots of room for improvement, which is fine.

Did I say working? Well, yes and no.

Working as in compiling and running, yes.

Working as in playing the game and catching the murderer, no.

Read about the [final result here](/blog/results).
