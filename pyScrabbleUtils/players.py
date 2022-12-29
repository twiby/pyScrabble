import sys
import numpy as np
from pyScrabbleUtils import scrabbleBoard as sb



class Players(object):
	def __init__(self, board, nPlayers=1):
		self.board = board
		self.players = [Player(board, name="player "+str(n+1)) for n in range(nPlayers)]
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

class Player(object):
	def __init__(self, board, name=""):
		self.board = board
		self.score = 0
		self.set = []
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

		words={w for w in self.board.wordFinder.get_anagrams(
			"".join(self.set),
			nb_letters=nLettersPossible,
			**self.getConstraints(x,y))}
		for w in words:
			wordObj = sb.Word(x,y, horizontal=False, word=w)
			wordScore = self.board.getScore(wordObj.replaceJoker(self.set, self.board))
			if self.board.allWordsExist(wordObj) and wordScore > bestWordScore:
				bestWordScore = wordScore
				bestWord = wordObj
		return bestWord, bestWordScore

	def findBestWord(self, printResult=False):
		if self.board.isEmpty():
			# First move of the game
			bestWordBase = self.board.wordFinder.get_best_first_play("".join(self.set), self.board.serialize());
		else:
			bestWordBase = self.board.wordFinder.get_best_play("".join(self.set), self.board.serialize());

		if bestWordBase == None:
			if printResult:
				print("no words found")
			return None

		bestWord = sb.Word(bestWordBase.coord[0], bestWordBase.coord[1], horizontal=not bestWordBase.vertical, word=bestWordBase.word.lower())
		bestScore = bestWordBase.score
		bestWord.replaceConstraint(self.board.tiles)
		
		if printResult and bestWord!=None:
			print("best word : "+str(bestWord)+" at ("+str(bestWord.x)+","+str(bestWord.y)+") horizontal:"+str(bestWord.horizontal)+" for "+str(bestScore)+" points")

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
		letters = np.array([self.board.tiles[i,y].letter for i in indices])
		if horizontal:
			self.board.tiles = self.board.tiles.transpose()
		indices = indices - x
		forward_constraints = np.where(indices >= 0)[0]
		letters = letters[forward_constraints]
		indices = indices[forward_constraints]
		return {"constraint_indices": indices, "constraint_letters": letters}
