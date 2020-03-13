import os
import sys
from random import shuffle

import pyScrabble.scrabbleBoard as sb


board = sb.Board()
board.print()
board.playOneTurn()
board.print()
board.playOneTurn()
board.print()
board.playOneTurn()
board.print()

# nTot = 10000
# scrabbleFound = 0
# for n in range(nTot):
# 	sys.stdout.write(str(n))
# 	sys.stdout.write('\r')
# 	sys.stdout.flush()
# 	temp = {w for w in board.players.wordTree.getAllAnagrams(board.setOfLetters[:7])}
# 	if temp == set():
# 		pass
# 	else:
# 		scrabbleFound += 1
# 	shuffle(board.setOfLetters)

# print("found a scrabble possible",scrabbleFound,"out of",nTot)
# print("whch is like",scrabbleFound/nTot*100,"%")