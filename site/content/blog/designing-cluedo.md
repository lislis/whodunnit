+++
author = "lislis"
title = "Designing Cluedo"
date = "2025-03-13"
description = "How does one go about designing Cluedo in Prolog?"
tags = [
  "cluedo",
  "prolog"
]
+++

If you have no idea what Cluedo is about, I invite you to either read [my post](/blog/related-work/) about it or the [Wikipedia article](https://en.wikipedia.org/wiki/Cluedo). Some basic knowledge is sufficient because I'll make changes to the gameplay to fit my idea for the game.

---

First things first in Prolog, we need to define the knowledge database. That is a list of facts that we want to be true and that the user can use to fact check against.

Our knowledge databse will roughly consist of two parts: the that are always true, like definitions of the suspects, weapons and locations, and the predicates (Prolog functions) that we need to define the game. The other part is the randomly generated one, where the clues are arranhegd so that every playthrough will be different.

Before we can generate the knowledge database with a different program, we need to know what we want to generate. So we'll start with a game fully handwritten.

## Base facts

Cluedo works with three categories: suspects, weapons and locations. I'll be using the standard Cluedo options listed on Wikipedia here.

Those are easy to represent in Prolog, and will also always be present in each variation of the scenario.

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

## Predicates

Next, let's look at the predicates we need. We need to mark the `murderer/1`, `murder_weapon/1` and `murder_location/1`, but this will be handled from the Rust side when a new game starts.

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

This also establishes the main premise and game mechanic: the player will question one suspect for information about another suspect.

If the interrogated suspect is innocent, they will give what I call `clue`, another innocent suspext with a weapon or location that is not related to the murder, giving them an alibi.

If the player questions the murderer, they will give what I call `false_clue`r: an innocent suspect seen with a murder weapon or location or something that is already part of an alibi, thus contradicting previous statements.

With this, the player should be able to deduct the murderer after a few rounds of questions. Hopefully 🤞

## Procedural parts

The base facts and predicates will (for now) always be the same. The dynamic parts, that will make every playthrough different, need to be generated.

At the start of a new game the `murderer/1`, `murder_weapon/1` and `murder_location/1` need to be filled in from the list of suspects, weapons and locations.

Then, as the player asks `question`s every round, the `alibi`s need to be added as well, to mark progress. In a pure Prolog session, this could be achieved via `assert` but I learned about this feature too late. Which is also good because it would have made me question the entire concept of this projects even harder even earlier.
