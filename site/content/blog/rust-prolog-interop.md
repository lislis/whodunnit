+++
author = "lislis"
title = "Rust-Prolog interop"
date = "2025-03-04"
description = "A short trip down the rabbit hole of making two things talk to each other"
tags = [
    "rust",
    "prolog",
    "interop",
]
+++

One of the fundamental steps in the entire project is to be able for the Rust programm to interact with the Prolog interpreter.

Rust offers `std::process::Command` to call out to external programs, which seemed like a fair enough start. (This would mean that the Prolog interpreter would need to be installed seperately by the user. But I decided to leave nice packaging up to final stage for future me :tm:)

While just calling out to Prolog ([scryer-prolog](https://github.com/mthom/scryer-prolog) in this case, a Rust based implementation) worked just fine. The base example however led to just starting the interpreter's own interactive sessino interface inside my Rust interface.

Technically, this is not a problem, since working within a Prolog session seemed sensible anyway. I do however need to have some game logic in Rust running in between the user input and Prolog interactions. And in this scenario I could not capture that.

For this I need to figure out how to "just" run the interpreter in a seperate background process and have Rust communicate between that and the user's input from stdin. This led me down a steep and deep rabbit hole, because while Rust offers planty of crates to abtract over  `std::process::Command` and adding functionality to it, the problem was Prolog.

## Capturing output from Prolog

Turns out that the default and really only supported usecase for Prolog is its very interactive mode.

All kinds of writing to streams, files and whatnot assume that you are in the session and want to write out your results or your history. Another program calling into Prolog and capturing its output just seems to not be supported? (Or I didn't find it?)

So, with no easy way to get I wanted, the first priority became just making it work somehow.

More online searching yielded [this result from Stack overflow](https://stackoverflow.com/questions/25467090/how-to-run-swi-prolog-from-the-command-line) proposing the idea that you can run the prolog interpreter, load a knowledge base file and then immediately `halt` (aka exit) the interpreter. This would then return to the parent program, my Rust program.

Next, I could set a `goal`, a Prolog query, to evaluate and write its outcome out to a file (as if I were in interactive mode) ([Mostly relying on this thread](https://swi-prolog.discourse.group/t/capture-standard-output-from-write-1-predicate-in-c/8523)).

Sounds good enough, but turns out scryer-prolog does not implement the needed commandline features. For this reason I switched to [SWI-Prolog]() (a more mature implementation, lovingly called swipl). I was also surprised and quickly excited to find there is a Rust crate providing bindings, called [swipl-rs](), but was just as quickly hit with reality that this crate [does not support loading pre-written Prolog files](https://github.com/terminusdb-labs/swipl-rs/issues/3)...

In the end my working solution consists of this: Having the knowledge database file (for now pre-written, later generated). Calling the SWI-Prolog with a oneliner to load this file, run a goal that writes its output to a file, then halts the interpreter. Then call cat on the output file to read the result back into my Rust program.

Luckily `std::process::Command` has the option of using `spawn` vs `output` so I can choose to wait for a systemcall to return and make sure to not run into race-conditions (or having to deal with async/await).

## Demo

This video demonstrates the first working interation of the above plan.

`test.pl` contains a simple knowledge base from [Learn Prolog Now!](https://lpn.swi-prolog.org/lpnpage.php?pagetype=html&pageid=lpn-htmlse2). The Rust code promps the user for a `goal` and then puts together the one-liner calling into Prolog and writing to a file. The file's output is then displayed. To make it more obvious that the output comes from said file, I'm `cat`ing into debug.txt several times.

Written out the one-liner would look something like this (assuming the goal is `playsAirGuitar(X)`):

``` sh
swipl -s test.pl -g "playsAirGuitar(X), open('debug.txt', write, Stream, []), format(Stream, X, [])." -g halt.
```

{{< rawhtml >}}
<video controls muted>
<source type="video/mp4" src="../../videos/rust-prolog-interaction.mp4"></source>
<a href="../videos/rust-prolog-interaction.mp4>Find the video file here</a>
</video>
{{< /rawhtml >}}
