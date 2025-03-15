---
title: "About Whodunnit"
draft: false
---


Whodunnit is an attempt to procedually generate [whodunit](https://en.wikipedia.org/wiki/Whodunit)-type game scenarios with a commandline/ text-based interface for a player to solve.

The unusual technical choice of using Prolog and Rust for this stems from my wish to learn the former and get better at the latter.

## Motivation

I'm interested in different technologies and using them in playful ways. I've been intrigued by logic programming as a concept and programming paradigm since I first heard about it but never had the time to really dig into it. At the same time I was occasionally thinking about what kind of game I could make within the constrains and possibilities offered by a logic programming language such as Prolog.

When remakes of classic Whodunit-genre films Murder on the Orient Express, Death on the Nile (based on the [Agatha Christie](https://en.wikipedia.org/wiki/Agatha_Christie) novels) and, more originally, Knives Out hit theaters a few years back, I realized it's the perfect setup for a logic programming game.

[Whodunit](https://en.wikipedia.org/wiki/Whodunit) follows a simple concept: a crime, likely murder, takes place. All potential suspects are known and, usually, confined within a physical space. The investigator of the crime is an outside, neutral person, who has to single out the culprit from the suspects using nothing but logic to put together the clues, alibis and motives. The audience knows and learns the clues together with the investigator and, in an ideal scenario, is able to deduct who the culprit is through their own thinking, thus creating an entertaining and interesting challenge to akin to participating.

Of course the idea isn't new. The board game [Cluedo](https://en.wikipedia.org/wiki/Cluedo) successfully makes use of exactly that setup. The free online game [Murdle](https://murdle.com/) builds up on that. I want to try implementing my own version using Prolog.

On the writing of 'Whodunit': Wikipedia lists both versions, with one and two 'n'. For the sake of more drama, I chose to go with 'Whodunnit' for this project specifically. And usually refer to the concept or genre itself as 'Whodunit'. In case you were wondering.


## Description and Plan

Whodunnit is a text-adventure-style murder-mystery game that is played on the commandline.

The player acts in the role of an investigator for the presented crime scenario through text. They can choose different options to approach the investigation, such as questioning suspects, inspecting locations and so forth. Different options will yield clues. The clues allow the player to deduce who of the suspects is the culprit.

Upon starting a new game, a scenario is generated in the background. A scenario consists of a crime, several suspects, their motives, possible weapons and locations. The relationship between these elements is procedurally generated and put down in a fact database. The fact-database is a valid Prolog program.

The player however cannot access all facts at once. They have to reveal the facts one after the other through the game. Each interaction is represented as a Prolog query against the fact database and evaluated and kept track of. When the player is able to deduct the culprit and verify the culprit-fact then the case is considered solved and the game ends.

The [Blog](/blog) of this page represents a development diary. Feel free to browse posts of interest.

Click [here for the current state of Wodunnit](/blog/results/) and [here for overall project reflection](/blog/reflection/).
