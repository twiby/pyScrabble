import os
import pyScrabble.scrabbleTree as ps
from pyScrabble import scrabbleBoard as sb

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
		self.set = []
		self.wordTree = wordTree
		self.updateSet()

	def updateSet(self):
		self.set += self.board.getNewLetters(7-len(self.set))

	def getStatus(self):
		status = ''
		status += str(self.set)
		status += ' current score: '
		status += str(self.score)
		return status

	def playOneTurn(self):
		bestWordScore = 0
		if self.board.log == []:
			# First move of the game
			wordList = list(self.wordTree.getAllAnagrams(self.set))
			if wordList == []:
				wordList = list(self.wordTree.getWordsFrom(self.set))
			wordList = list({w.asString() for w in wordList})
			if wordList == []:
				self.board.setOfLetters += self.set
				self.set = self.board.getNewLetters(7)
				return
			for w in wordList:
				for y in range(8-len(w), 8):
					wordObj = sb.Word(7,y, word=w)
					wordScore = self.board.getScore(wordObj.replaceJoker(self.set))
					if wordScore > bestWordScore:
						bestWord = wordObj
						bestWordScore = wordScore

		self.score += self.board.play(bestWord, self.set)
		self.updateSet()


