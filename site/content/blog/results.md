+++
author = "lislis"
title = "Discussion of Results"
date = "2025-03-15"
description = "Overview and brief discussion of the results and potential future work"
tags = [
  "documentation"
]
+++

## Results

So, what's the state of Wodunnit as of right now? It's complicated.

There is a small screen recording that shows what is currently working at the end of the post.

When you run the Rust project a game file is created in `tmp` with the current timestamp. It contains the knowledge snippets predefined in text files in `game`. At the very end the randomly generated murder case is added.

Once you enter the `loop` (the investigation if you will) you can write some Prolog to query the generated knowledge base.

Ideally, obviously, this would be hidden behind a nicer terminal interface and then parsed and passed to Prolog behind the scenes. But I digress.

Notice in the examples how the simple goals with predicates that take one argument (like `suspect(X)`, `murderer(X)` and `weapon(X)`) work and produce the expected Prolog output in the debug.txt file.

All predicates with more than one arguement (`question/3` in the clip, but also `clue/2` and others) do not produce output that Prolog can write to the debug.txt file.

And I'm at a loss as to why that is? To me and my current Prolog knowledge it just doesn't make sense. I'm pretty sure the answer is something small that is obvious to someone with more Prolog experience, and I'm sure with more time and fiddling I could make it work.

## Future work

The most obvious next step in this project ist to understand and fix the interoperability issue. It's likely in the Prolog code but needs further investigation.

After that the solution to the main game generation logic as outlined [here](blog/designing-whodunnit/#the-issue-with-alibi) should be tackled to make the Prolog side of this as robust as possible.

Afterwards the Rust prompting interface and overall code structure needs to be improved, some pointers on where to start [are given here](/blog/glue-rust/).

Once these are steps are completed, I think one could think about a first release and have users playtest.

{{< rawhtml >}}
<video controls muted>
<source type="video/mp4" src="../../videos/final_result.mp4"></source>
<a href="../videos/final_result.mp4>Find the video file here</a>
</video>
{{< /rawhtml >}}
