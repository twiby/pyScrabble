# pyScrabble
a utility for scrabble.

pyScrabble will download words from the french ODS8 dictionnary and create our custom data structures, saving them both to files, if they are missing. 
Default usage is an interface where you can fill in the board and your letters and it gives you the best play.
Auto usage, for testing and fun, is
```
./pyScrabble --auto
```
for help on the different options, use
```
./pyScrabble -h
```


# Personalisation

For those who want to try their own algorithms, data structures and utils are defined in scrableTree.py 
(getWord and getAllAnagrams are the main functions used outside)

The algo for searching best move is in pyScrabbleUtils/players.py in the method Player.findBestWord
