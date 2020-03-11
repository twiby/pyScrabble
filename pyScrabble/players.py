import os
import pyScrabble.scrabbleTree as ps
from pyScrabble import scrabbleBoard

folder = os.path.dirname(os.path.realpath(__file__)) + "/../"

### Get all the words of scrabble in a text file
if not os.path.isfile(folder + "scrabbleWords.txt"):
	from pyScrabble import scrabbleDict as sd
	sd.writeScrabbleWordsToNewFile(folder + "scrabbleWords.txt")


### Organize the data in a tree
if not os.path.isfile(folder + "scrabble.tree"):
	scrabbleTree = ps.createTree(folder + "scrabbleWords.txt")
	scrabbleTree.save(folder + "scrabble.tree")

# scrabbleTree.children[0].children[-1].print()
# scrabbleTree.children[0].children[-1].children[0].print()
# print(scrabbleTree.getWord("azure").asString())


# res = {x.asString() for x in scrabbleTree.getWordsFrom("manger0i")}
# print(res)
# print([scrabbleTree.getWord(x).isWord for x in res])
# print({x.asString() for x in scrabbleTree.getAllAnagrams("manger0i")})


class Players(object):
	def __init__(self, board, nPlayers=1):
		self.board = board
		self.wordTree = ps.loadTree(folder + "scrabble.tree")
		self.players = [Player(board, self.wordTree) for _ in range(nPlayers)]

	def playOneTurn(self):
		for player in self.players:
			player.playOneTurn()

class Player(object):
	def __init__(self, board, wordTree):
		self.board = board
		self.score = 0
		self.set = ''
		self.wordTree = wordTree
		self.updateSet()

	def updateSet(self):
		self.set = self.board.getNewLetters(7-len(self.set))

	def getStatus(self):
		status = ''
		status += str(self.set)
		status += ' current score: '
		status += str(self.score)
		return status

	def playOneTurn(self):
		word = self.wordTree.getWordsFrom(self.set)
		self.score += self.board.play(scrabbleBoard.Word(7,6,horizontal=True, word="ewe"))
		self.score += self.board.play(scrabbleBoard.Word(7,9,horizontal=False, word="saline"))
		self.score += self.board.play(scrabbleBoard.Word(9,7,horizontal=True, word="atlas"))
		self.score += self.board.play(scrabbleBoard.Word(11,10,horizontal=False, word="et"))
