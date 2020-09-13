import os
import sys
import numpy as np
import pyScrabbleUtils.scrabbleTree as ps
from pyScrabbleUtils import scrabbleBoard as sb

folder = os.path.dirname(os.path.realpath(__file__)) + "/../"

### Get all the words of scrabble in a text file
if not os.path.isfile(folder + "scrabbleWords.txt"):
	from pyScrabbleUtils import scrabbleDict as sd
	sd.writeScrabbleWordsToNewFile(folder + "scrabbleWords.txt")


### Organize the data in a tree
if not os.path.isfile(folder + "scrabble.tree"):
	scrabbleTree = ps.createTree(folder + "scrabbleWords.txt")
	scrabbleTree.save(folder + "scrabble.tree")



class Players(object):
	def __init__(self, board, nPlayers=1):
		self.board = board
		self.wordTree = ps.loadTree(folder + "scrabble.tree")
		self.wordTree.addWord("ud")
		self.wordTree.addWord("woh")
		self.players = [Player(board, self.wordTree, name="player "+str(n+1)) for n in range(nPlayers)]
		self.finisher = None

	def playOneTurn(self, show=False):
		for player in self.players:
			player.playOneTurn(show=show)
			if len(player.set)==0:
				break

	def pickLetters(self):
		for pl in self.players:
			pl.updateSet()

	def done(self):
		for p in self.players:
			if len(p.set)==0:
				self.finisher = p
				return True
		for p in self.players:
			if not p.done:
				return False
		return True

	def addWords(self, words):
		for word in words:
			self.wordTree.addWord(word)

class Player(object):
	def __init__(self, board, wordTree, name=""):
		self.board = board
		self.score = 0
		self.set = []
		self.wordTree = wordTree
		self.done = False
		self.name = name

	def updateSet(self):
		self.set += self.board.getNewLetters(7-len(self.set))

	def getStatus(self):
		status = self.name + " "
		status += str(self.set)
		status += ' current score: '
		status += str(self.score)
		return status

	def findBestWordAt(self, x, y):
		bestWord = None
		bestWordScore = 0

		if x!=0 and self.board.tiles[x-1,y].letter!=None:
			return bestWord, bestWordScore

		n = 1
		nConstraints = 0
		nLettersPossible = []
		while n < 8:
			if x+n+nConstraints-1<15 and self.board.tiles[x+n+nConstraints-1,y].letter!=None:
				nConstraints += 1
				continue
			elif x+n+nConstraints<15 and self.board.tiles[x+n+nConstraints,y].letter!=None:
				n += 1
				continue
			try:
				wObj = sb.Word(x,y, word='0'*(n+nConstraints), horizontal=False)
			except sb.WordError:
				break
			if self.board.isValidMove(wObj):
				nLettersPossible.append(n+nConstraints)
			n += 1

		words={w.asString() for w in self.wordTree.getAllAnagrams(
			self.set,
			nLetters=nLettersPossible,
			**self.getConstraints(x,y))}
		for w in words:
			wordObj = sb.Word(x,y, horizontal=False, word=w)
			wordScore = self.board.getScore(wordObj.replaceJoker(self.set, self.board))
			if self.board.allWordsExist(wordObj) and wordScore > bestWordScore:
				bestWordScore = wordScore
				bestWord = wordObj
		return bestWord, bestWordScore

	def findBestWord(self, printResult=False):
		bestWords = []
		bestScores = []
		if self.board.isEmpty():
			# First move of the game
			bestScores = [0]
			wordList = list(self.wordTree.getAllAnagrams(self.set))
			wordList = list({w.asString() for w in wordList})
			for w in wordList:
				for y in range(8-len(w), 8):
					wordObj = sb.Word(7,y, word=w)
					wordScore = self.board.getScore(wordObj.replaceJoker(self.set, self.board))
					if wordScore > bestScores[0]:
						bestWords = [wordObj]
						bestScores = [wordScore]

		else:
			for horizontal in [False, True]:
				for x in range(15):
					for y in range(15):
						bw, bs = self.findBestWordAt(x,y)
						if bw!=None:
							if horizontal:
								bw = sb.Word(bw.y, bw.x, horizontal=True, word=str(bw))
							bestWords.append(bw)
							bestScores.append(bs)

						
				self.board.tiles = self.board.tiles.transpose()
		

		if bestWords == []:
			if printResult:
				print("no words found")
			return None
		bestWords = np.array(bestWords)
		bestScores = np.array(bestScores)
		bestWord = bestWords[np.argmax(bestScores)]
		
		if printResult and bestWord!=None:
			bestWordScore = np.max(bestScores)
			print("best word : "+str(bestWord)+" at ("+str(bestWord.x)+","+str(bestWord.y)+") horizontal:"+str(bestWord.horizontal)+" for "+str(bestWordScore)+" points")

		return bestWord

	def playOneTurn(self, show=False):
		bestWord = self.findBestWord(printResult=show)

		if bestWord==None:
			if len(self.board.setOfLetters) == 0:
				self.done = True
				return
			self.board.setOfLetters += self.set
			self.set = self.board.getNewLetters(7)
		else:
			self.score += self.board.play(bestWord, self.set)
			self.updateSet()
	
	def getConstraints(self, x, y, horizontal=False):
		if horizontal:
			x,y = y,x
			self.board.tiles = self.board.tiles.transpose()
		indices = np.where(np.array([self.board.tiles[i,y].letter for i in range(15)]) != None)[0]
		letters = [self.board.tiles[i,y].letter for i in indices]
		if horizontal:
			self.board.tiles = self.board.tiles.transpose()
		return {"constraintIndices": indices-x, "constraintLetters": letters}
