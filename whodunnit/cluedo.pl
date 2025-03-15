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


murderer(mrsWhite).
murder_weapon(dagger).
murder_location(study).


innocent(X) :- suspect(X),
               \+ murderer(X).

not_murder_weapon(X) :- weapon(X),
             \+ murder_weapon(X).

not_murder_location(X) :- location(X),
                \+ murder_location(X).

:- dynamic alibi/1.

alibi(X) :- innocent(X);
            not_murder_location(X);
            not_murder_weapon(X).


clue(X, Y) :- innocent(X), location(Y), not_murder_location(Y), \+ alibi(Y).
clue(X, Y) :- innocent(X), weapon(X), not_murder_weapon(Y), \+ alibi(Y).

false_clue(X, Y) :- innocent(X), ( murder_weapon(Y); (weapon(Y), alibi(Y)) ).
false_clue(X, Y) :- innocent(X), ( murder_location(Y); (location(Y), alibi(Y)) ).

accuse(X, Y, Z) :- murderer(X),
                   murder_weapon(Y),
                   murder_location(Z).

question(X, Y, Z) :- innocent(X) -> clue(Y, Z); false_clue(Y, Z).
