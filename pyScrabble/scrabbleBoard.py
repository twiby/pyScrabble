import numpy as np
from random import shuffle
from pyScrabble import constants
from pyScrabble import players as pl

class TileError(Exception):
	pass
class WordError(Exception):
	pass


class Word(object):
	class Letter(object):
		def __init__(self,x,y,c):
			self.x = x
			self.y = y
			self.c = c
			if self.x >14 or self.y > 14 or self.x<0 or self.y<0:
				raise WordError("letter outside map")

	def __init__(self, x, y, horizontal=True, word=[]):
		self.horizontal = horizontal
		self.xI = int(not self.horizontal)
		self.letters = []
		for n in range(len(word)):
			self.letters.append(
				self.Letter(x + n*self.xI,
							y + n*(1-self.xI),
							word[n]))

	def __str__(self):
		return ''.join([l.c for l in self.letters])

	def __len__(self):
		return len(self.letters)

	def getLetters(self):
		for letter in self.letters:
			yield letter.x, letter.y, letter.c

	def numLetter(self):
		return len(self.letters)

	def getOtherWordsFormed(self, board):
		for letter in self.letters:
			if board.tiles[letter.x, letter.y].letter!=None:
				continue
			foundAnotherWord = False
			posPre = [letter.x, letter.y]
			while posPre[self.xI]>0 and board.tiles[posPre[0]-(1-self.xI),posPre[1]-self.xI].letter!=None:
				foundAnotherWord = True
				posPre[self.xI] -= 1
			posPost = [letter.x, letter.y]
			while posPost[self.xI]<15 and board.tiles[posPost[0]+(1-self.xI), posPost[1]+self.xI].letter!=None:
				foundAnotherWord = True
				posPost[self.xI] += 1

			if foundAnotherWord:
				word = ''; n=0;
				while posPre[0]+n*(1-self.xI) != posPost[0] or posPre[1]+n*self.xI != posPost[1]:
					word += letter.c if board.tiles[posPre[0]+n*(1-self.xI), posPre[1]+n*self.xI].letter==None \
						else board.tiles[posPre[0]+n*(1-self.xI), posPre[1]+n*self.xI].letter
					n+=1
				word += letter.c if board.tiles[posPre[0]+n*(1-self.xI), posPre[1]+n*self.xI].letter==None \
					else board.tiles[posPre[0]+n*(1-self.xI), posPre[1]+n*self.xI].letter
				yield Word(posPre[0], posPre[1], horizontal=not self.horizontal, word=word)






class Board(object):
	class Tile(object):
		def __init__(self, wordFactor, letterFactor, letter=None):
			self.wordFactor = wordFactor
			self.letterFactor = letterFactor
			self.letter = letter

		def playTile(self, c):
			if self.letter != None and self.letter != c:
				raise TileError("Tile not compatible !")
			elif self.letter == c:
				return False
			else:
				self.letter = c
				self.wordFactor = 1
				self.letterFactor = 1
				return True

	def __init__(self, nPlayers=1):
		wordFactorGrid = np.pad(constants.wordFactorGrid, ((0, 7),(0, 7)), 'reflect')
		letterFactorGrid = np.pad(constants.letterFactorGrid, ((0, 7),(0, 7)), 'reflect')
		self.tiles = np.array([[self.Tile(wordFactorGrid[x,y], letterFactorGrid[x,y]) for x in range(15)] for y in range(15)], dtype=object)
		self.setOfLetters = constants.setOfLetters
		shuffle(self.setOfLetters)

		self.log = []
		self.players = pl.Players(self, nPlayers)

		self.players.playOneTurn()
		
	def print(self):
		for x in range(15):
			str = ''.join([self.tiles[x,y].letter+" " if self.tiles[x,y].letter!=None else "_ " for y in range(15)])
			print(str)
		print()
		for player in self.players.players:
			print(player.getStatus())

	def allWordsFormed(self, word):
		yield word
		yield from word.getOtherWordsFormed(self)

	def getWordScore(self, word):
		score = 0
		wordFactor = 1
		for x,y,c in word.getLetters():
			wordFactor *= self.tiles[x,y].wordFactor
			score += self.tiles[x,y].letterFactor * constants.points[c]
		return score * wordFactor

	def getScore(self, word):
		return sum(self.getWordScore(w) for w in self.allWordsFormed(word))

	def wordExists(self, word):
		if self.players.wordTree.getWord(str(word))==None:
			return False
		else:
			return True

	def allWordsExist(self, word):
		for w in self.allWordsFormed(word):
			if self.wordExists(w)==False:
				raise(WordError(str(w)+" does not exist"))
		return

	def isValidMove(self, word):
		if len(self.log)==0:
			passThroughMiddle = False
			for x,y,_ in word.getLetters():
				if x==7 and y==7:
					passThroughMiddle = True
			if passThroughMiddle:
				return
			else:
				raise WordError("The first word must pass by middle tile")
		if len([w for w in self.allWordsFormed(word)])>1:
			return
		else:
			usesExistingTile = False
			for x,y,_ in word.getLetters():
				if self.tiles[x,y].letter is not None:
					usesExistingTile = True
			if usesExistingTile:
				return
			else:
				raise WordError("The word must interact with other tiles in some way")

	def play(self, word, set):
		'''SHOULD NOT USE A WORD, SHOULD USE ANOTHER OBJECT RELATED TO A PLAYER, FOR JOKER+POP REASONS'''
		self.isValidMove(word)
		self.allWordsExist(word)
		for x,y,c in word.getLetters():
			if self.tiles[x,y].playTile(c):
				try:
					set.remove(c)
				except ValueError:
					set.remove("0")
					c = "0"
		self.log.append(word)
		score = self.getScore(word)
		return score + 50*int(set==[])

	def getNewLetters(self, nNewLetters):
		newLetters = []
		if nNewLetters>=len(self.setOfLetters):
			newLetters, self.setOfLetters = self.setOfLetters, newLetters
		else:
			shuffle(self.setOfLetters)
			newLetters = [self.setOfLetters.pop() for _ in range(nNewLetters)]
		return newLetters