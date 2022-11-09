# Rating Top Smash Ultimate Players

This small project aims to rate every game found in the [smashdata.gg](https://smashdata.gg) database for Super Smash Bros. Ultimate using the TrueSkill algorithm developed by Microsoft.

We are using the SQLite Database kindly provided by smashdata, last updated on 2022-11-06. [(Link)](https://github.com/smashdata/ThePlayerDatabase/releases/tag/v2022.11.06)  
There are over 470,000 unique players and over 6,8 million games in the database as of right now.

Please note, that while there is a large overlap, these results may not reflect reality accurately.

## Top 50

For the full dataset, please see [results.csv](./data/results.csv).  
There are over 470,000 players in the results, here is the Top 50.

| Player | Rating | Wins | Losses |
| ------ | ------ | ---- | ------ |
MkLeo | 45.36 | 964 | 161 | 
あcola | 44.44 | 178 | 24 | 
S | 44.38 | 131 | 14 | 
Light | 43.90 | 1050 | 182 | 
Sisqui | 43.73 | 1009 | 157 | 
Nairo | 43.33 | 219 | 55 | 
Kola | 43.26 | 1396 | 274 | 
Sonix | 43.25 | 2623 | 548 | 
Riddles | 43.23 | 1231 | 293 | 
Ouch!? | 42.95 | 694 | 130 | 
Kum Hia Nao | 42.80 | 979 | 132 | 
Kurama | 42.79 | 1319 | 229 | 
Glutonny | 42.76 | 1264 | 194 | 
Tyler | 42.61 | 858 | 88 | 
Alec Kennison | 42.51 | 49 | 1 | 
Shuton | 42.39 | 635 | 148 | 
Lui$ | 42.37 | 2474 | 549 | 
Tea | 42.27 | 453 | 122 | 
Zomba | 42.23 | 1458 | 701 | 
Zinoto | 42.04 | 876 | 120 | 
Chag | 42.02 | 2710 | 787 | 
Supahsemmie | 41.94 | 356 | 68 | 
IcyMist | 41.82 | 955 | 267 | 
Onin | 41.72 | 1095 | 518 | 
Elexiao | 41.69 | 262 | 57 | 
Leon | 41.66 | 1716 | 388 | 
Jdizzle | 41.62 | 1071 | 199 | 
ApolloKage | 41.60 | 1388 | 398 | 
Kreeg | 41.59 | 1239 | 460 | 
Megafox | 41.59 | 389 | 28 | 
SHADIC | 41.58 | 631 | 172 | 
Quidd | 41.57 | 746 | 233 | 
omega | 41.54 | 1460 | 433 | 
Skar | 41.52 | 652 | 158 | 
Sparg0 | 41.51 | 1647 | 371 | 
Lemmon | 41.47 | 675 | 205 | 
Tarik | 41.46 | 735 | 226 | 
MuteAce | 41.40 | 1300 | 303 | 
Big D | 41.35 | 695 | 200 | 
Lima | 41.28 | 825 | 188 | 
Space | 41.23 | 460 | 103 | 
Peabnut | 41.18 | 607 | 144 | 
Bloom4Eva | 41.14 | 1140 | 297 | 
Secret | 41.13 | 197 | 37 | 
Dabuz | 41.12 | 990 | 302 | 
Rayquaza07 | 41.08 | 217 | 13 | 
Geist | 41.07 | 796 | 106 | 
Anathema | 41.01 | 515 | 126 | 
JDV | 40.98 | 726 | 233 | 
Doomblaze | 40.98 | 232 | 9 | 

(Note that ratings are not rounded |  but cut off.)

## Predicting Games

Using these ratings, we can not only rank players, but also predict future games.

Using a quick example, predicting the outcome of MKLeo (rank: ~45.36, #1) vs Dabuz (rank: ~41.12, #45) yields the following expected scores:

- MKLeo: ~**0.771**
- Dabuz: ~**0.229**

Because in Smash Ultimate tournaments draws are ruled out, this means that MKLeo has roughly a 77.1% chance to win, leaving Dabuz with a 22.9% chance.

You can play around with these values in the `predict_game` function in [`main.rs`](./src/main.rs).

## Links

- [smashdata.gg](https://smashdata.gg)
- [TrueSkill](https://www.microsoft.com/en-us/research/project/trueskill-ranking-system/)

Crates used:

- [skillratings](https://crates.io/crates/skillratings)
- [rusqlite](https://crates.io/crates/rusqlite)
- [csv](https://crates.io/crates/csv)
- [serde](https://crates.io/crates/serde)
- [hashbrown](https://crates.io/crates/hashbrown)