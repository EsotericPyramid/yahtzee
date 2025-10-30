# Yahtzee

A basic implementation of Yahtzee in Rust.

## How to Play

Every round, you will be prompted with 5 dice to reroll
into one of a number of patterns. You have 2 rerolls and
they are specified by listing whether or not to reroll each die (y / n).
So to reroll the first, third, and fifth dice, you would enter:

y
n
y
n
y

After using all rerolls, you will be prompted to select one of
the remaining patterns to score the dice with. Scoring is the
same as standard Yahtzee.

After 13 rounds, you recieve your total score