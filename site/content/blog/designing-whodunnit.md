+++
author = "lislis"
title = "Designing Whodunnit"
date = "2025-03-13"
description = "How does one go about designing Cluedo in Prolog?"
tags = [
  "game design",
  "cluedo",
  "prolog",

]
+++

If you have no idea what Cluedo is about, I invite you to either read [my post](/blog/related-work/) about it or the [Wikipedia article](https://en.wikipedia.org/wiki/Cluedo). Some basic knowledge is sufficient because I'll make changes to the gameplay to fit my idea for the game.

My vision for Whodunnit, while borrowing a lot from Cluedo, has a bigger focus on interrogating the suspects as an outside person. More like in classic Agatha Christie mysteries, like Death on the Nile.

I want to design the interface in a way that feels like you're asking one suspect about other suspects's alibis.

The innocent suspects will answer truthfully, the murderer will lie. This idea I stole from one kind of clues you get in Murdle, I quite like it because it's very difficult for my brain to sort this out, but feels really rewarding when I get it right.

In the game I'd like to offer the player to keep track of the statements different suspects made. The final deduction is then up to the player and should then only be verified by the Prolog program.

So, let's get started and see how far we get.

## The knowledge base

First things first, we need to define the knowledge database. That is a list of facts that we want to be true and that the user can fact check against.

Our knowledge database will roughly consist of two parts: the that are always true, like definitions of the suspects, weapons and locations, and the predicates (Prolog functions) that we need to define the game mechanics.
The other part is the randomly generated one, where the facts are put together so that every playthrough will be different.

## Base facts

Cluedo works with three categories: suspects, weapons and locations. I'll be using the standard Cluedo options listed on Wikipedia as the starting point for Whodunnit.

These facts are easy to represent in Prolog, and will also always be present in each variation of the scenario.

``` prolog
suspect(missScarlett).
suspect(colonelMustard).
suspect(mrsWhite).
suspect(reverendGreen).
suspect(mrsPeacock).
suspect(professorPlum).

weapon(candlestick).
weapon(dagger).
weapon(leadPipe).
weapon(revolver).
weapon(rope).
weapon(wrench).

location(kitchen).
location(ballroom).
location(conservatory).
location(library).
location(billiardRoom).
location(study).
location(hall).
location(lounge).
location(diningRoom).

```

I ended up splitting these into different .txt files and writing out the Prolog file via Rustin the end. Because then the data input format is very simple and easily extended. Premature optimization? Obviously.

## Predicates

Next, let's look at the predicates (aka functionns) we need. We need to mark the `murderer/1`, `murder_weapon/1` and `murder_location/1`, but this will be handled from the Rust side when a new game starts.

Assuming these markers, I added some predicates for negation, to better point out the non-murder related facts.

``` prolog

innocent(X) :- suspect(X),
               \+ murderer(X).

not_murder_weapon(X) :- weapon(X),
             \+ murder_weapon(X).

not_murder_location(X) :- location(X),
                \+ murder_location(X).

```

For potentially solving the case in the end I defined

``` prolog

accuse(X, Y, Z) :- murderer(X),
                   murder_weapon(Y),
                   murder_location(Z).

```
which will only return `true` if suspect, weapon and location are their murder-variations. Easy.

## Main game logic

The main user interaction logic will be done through the `question/3` predicate and its clauses:

``` prolog

clue(X, Y) :- innocent(X), not_murder_location(Y), \+ alibi(Y).
clue(X, Y) :- innocent(X), not_murder_weapon(Y), \+ alibi(Y).

false_clue(X, Y) :- innocent(X), ( murder_weapon(Y); alibi(Y) ).
false_clue(X, Y) :- innocent(X), ( murder_location(Y); alibi(Y) ).


question(X, Y, Z) :- innocent(X) -> clue(Y, Z); false_clue(Y, Z).

```

This is my attempt to establish the main game mechanic: the player will question one suspect for information about another suspect.

If the interrogated suspect is innocent, they will give what I call `clue`, another innocent suspext with a weapon or location that is not related to the murder, and that has not yet an alibi.

If the player questions the murderer, they will give what I call `false_clue`: an innocent suspect seen with a murder weapon or location or something that already has an alibi, thus contradicting previous statements.

With this, the player should be able to deduct the murderer after a few rounds of questions. Hopefully 🤞

## Procedural parts

The base facts and predicates will (for now) always be the same. The dynamic parts, that will make every playthrough different, need to be generated.

At the start of a new game the `murderer/1`, `murder_weapon/1` and `murder_location/1` need to be filled in from the list of suspects, weapons and locations. I read these from .txt files into Rust, pick random options, and then write out the Prolog code into the knowledge base.

Then, as the player asks `question`s every round, the `alibi`s need to be added as well, to mark progress. In a pure Prolog session, this could be achieved via `assert` but I learned about this feature too late (It's how [Clues](https://github.com/asigdel29/Clues/blob/main/clues.pl) solves the collection of clues quite elegantly in interactive mode). Which is also good because it would have made me question the entire concept of this projects even harder even earlier.

## The issue with `alibi`

Avid readers may have noticed a flaw in my concept, that I also only fully realized way too late.

My plan was to generate the murder case up front and let Prolog figure out the rest as the player asks `question`s. But, in order to give out `clue` that stay consistent, I need to also define the "who was where with what" question up front to fill in all the blanks.

I didn't really account for that yet, as my planned steps were:
- generate knowledge base
- select murder scenario, write to knowledge base
- the run the loop that queries Prolog with this knowledge base
  - adds alibi to knowledge base for persistance after every loop
- user deducts murder scenario by looking at accumulated alibis
With the bit by bit addition of alibis magically making sense in the end.

What I really should have been planning to do:
- generate knowledge base
- select murder scenario, write to knowledge base
- extra pass to Prolog to figure our all other alibis, write to knowledge base
- the run the loop that queries Prolog with this knowledge base
  - fact checks knowledge base, no additional writing to knowledge base
- user deducts murder scenario by looking at outcomes of fact checks

It's a small difference that makes all the difference. But again brings us back to one of our initial problems of [Rust talking to Prolog](/blog/rust-prolog-interop/) and how tricky it is to get info out of Prolog... And in this case I'd need to get a list of _all_ the possibilities that Prolog can find for these alibis... It kind of breakes my brain tbh.
