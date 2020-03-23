# pyScrabble
a utility for scrabble.

The main.py will download words and create our custom data structures, saving them both to files, if they are missing. 
Default usage is an interface where you can fill in the board and your letters and it gives you the best play.
Auto usage, for testing and fun, is (for 3 players for example)
```
python main.py --auto --players 3
```
Only restriction : words with more than 3 constraints are not tried.


# Personalisation

For those who want to try their own algorithms, data structures and utils are defined in scrableTree.py 
(getWord and getAllAnagrams are the main functions used outside)

The algo for searching best move is in pyScrabble/players.py in the method Player.findBestWord