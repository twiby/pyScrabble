import os
import sys

import pyScrabble.scrabbleTree as ps
import pyScrabble.scrabbleBoard as sb

folder = os.path.dirname(os.path.realpath(__file__)) + "/"


### Get all the words of scrabble in a text file
if not os.path.isfile(folder + "scrabbleWords.txt"):
	from pyScrabble import scrabbleDict as sd
	sd.writeScrabbleWordsToNewFile(folder + "scrabbleWords.txt")


### Organize the data in a tree
if not os.path.isfile(folder + "scrabble.tree"):
	scrabbleTree = ps.createTree(folder + "scrabbleWords.txt")
	scrabbleTree.save(folder + "scrabble.tree")
else:
	scrabbleTree = ps.loadTree(folder + "scrabble.tree")

# scrabbleTree.children[0].children[-1].print()
# scrabbleTree.children[0].children[-1].children[0].print()
# print(scrabbleTree.getWord("azure").asString())



print({x.asString() for x in scrabbleTree.getAllAnagrams("mangerai")})
print({x.asString() for x in scrabbleTree.getWordsFrom("mangerai")})
board = sb.Board()
