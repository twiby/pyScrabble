import os
import sys
import numpy as np
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



class Players(object):
	def __init__(self, board, nPlayers=1):
		self.board = board
		self.wordTree = ps.loadTree(folder + "scrabble.tree")
		self.wordTree.addWord("ud")
		self.players = [Player(board, self.wordTree, name="player "+str(n+1)) for n in range(nPlayers)]
		self.finisher = None

	def playOneTurn(self):
		for player in self.players:
			player.playOneTurn()
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

	def findBestWord(self, printResult=False):
		bestWord = None
		bestWordScore = 0
		if self.board.isEmpty():
			# First move of the game
			wordList = list(self.wordTree.getAllAnagrams(self.set, nLetters=[7]))
			if wordList == []:
				wordList = list(self.wordTree.getAllAnagrams(self.set))
			wordList = list({w.asString() for w in wordList})
			for w in wordList:
				for y in range(8-len(w), 8):
					wordObj = sb.Word(7,y, word=w)
					wordScore = self.board.getScore(wordObj.replaceJoker(self.set, self.board))
					if wordScore > bestWordScore:
						bestWord = wordObj
						bestWordScore = wordScore

		else:
			for horizontal in [False, True]:
				for x in range(15):
					for y in range(15):
						if x!=0 and self.board.tiles[x-1,y].letter!=None:
							continue
						if printResult and bestWord != None:
							sys.stdout.write("best:"+str(bestWord.x)+","+str(bestWord.y)+": "+str(bestWord)+" ("+str(bestWordScore)+")  ((current "+str(x)+","+str(y)+"))    ")
							sys.stdout.write("\r")
							sys.stdout.flush()

						constraintLetters=[]
						constraintIndices=[]
						nLettersPossible =[]
						n=2
						while n-len(constraintIndices)<8 and x+n<16:
							if self.board.tiles[x+n-1,y].letter!=None:
								constraintLetters.append(self.board.tiles[x+n-1,y].letter)
								constraintIndices.append(n-1)
							elif x+n>=15 or self.board.tiles[x+n,y].letter!=None:
								n+=1
								continue
							wObj = sb.Word(x,y, word='0'*n, horizontal=False)
							if self.board.isValidMove(wObj):
								nLettersPossible.append(n)
							n+=1
						
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
								if horizontal:
									bestWord = sb.Word(bestWord.y, bestWord.x, horizontal=True, word=str(bestWord))
				self.board.tiles = self.board.tiles.transpose()
		
		if printResult and bestWord:
			print("best word : "+str(bestWord)+" at ("+str(bestWord.x)+","+str(bestWord.y)+") horizontal:"+str(bestWord.horizontal)+" for "+str(bestWordScore)+" points")
		elif printResult:
			print("no word found.")
		else:
			print()
		return bestWord

	def playOneTurn(self):
		bestWord = self.findBestWord(printResult=True)

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
