import sys
import numpy as np
from random import shuffle
from pyScrabbleUtils import constants
from pyScrabbleUtils import players as pl
import colorama

class TileError(Exception):
	pass
class WordError(Exception):
	pass

def computeScrabbleStats(nTot=10000):
	from random import shuffle
	board = Board()
	scrabbleFound = 0
	for n in range(nTot):
		sys.stdout.write(str(n))
		sys.stdout.write('\r')
		sys.stdout.flush()
		temp = {w for w in board.players.wordTree.getAllAnagrams(board.setOfLetters[:7], nLetters=[7])}
		if temp == set():
			pass
		else:
			scrabbleFound += 1
		shuffle(board.setOfLetters)

	print("found a scrabble possible",scrabbleFound,"out of",nTot)
	print("whch is like",scrabbleFound/nTot*100,"%")

def askYesNo(text):
	answer = None
	while answer not in ("y", "n"):
		answer = input(text+" [y/n]: ")
		if answer == "y":
			return True
		elif answer == "n":
			return False
		else:
			print("Please enter yes or no.")

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
		self.x, self.y = x, y
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

	def numLetters(self):
		return len(self.letters)

	def numTrueLetters(self, board):
		nTrueLetters = 0
		for x,y,c in self.getLetters():
			if board.tiles[x,y].letter == None:
				nTrueLetters += 1
		return nTrueLetters

	def replaceJoker(self, set, board):
		newWord = Word(self.letters[0].x, self.letters[0].y, horizontal=self.horizontal, word=str(self))
		for letter in newWord.letters:
			if letter.c not in set:
				boardLetter = board.tiles[letter.x, letter.y].letter
				if boardLetter == None:
					if '0' in set:
						letter.c = '0'
					else:
						raise WordError('unexpected letter :'+letter.c)
				else:
					letter.c = boardLetter
		return newWord

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
			while posPost[self.xI]<14 and board.tiles[posPost[0]+(1-self.xI), posPost[1]+self.xI].letter!=None:
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

	def __init__(self, nPlayers=1, show=False):
		wordFactorGrid = constants.wordFactorGrid.copy()
		letterFactorGrid = constants.letterFactorGrid.copy()
		self.tiles = np.array([[self.Tile(wordFactorGrid[x,y], letterFactorGrid[x,y]) for x in range(15)] for y in range(15)], dtype=object)
		self.setOfLetters = constants.setOfLetters
		shuffle(self.setOfLetters)

		self.log = []
		self.players = pl.Players(self, nPlayers)
		self.show = show

	def startAdviser(self):
		import pyScrabbleUtils.interface as i
		playing = True
		while playing:
			self = i.getScrabbleBoard(self)
			self.players.players[0].set = list(input("Enter your set of letters: "))
			bestWord = self.players.players[0].findBestWord(printResult=self.show)
			playing = askYesNo("Continue ?")
			if bestWord!=None:
				self.play(bestWord, self.players.players[0].set)
	
	def startDict(self):
		import pyScrabbleUtils.interface as i
		playing = True
		while playing:
			word = list(input("Enter your potential word: "))
			if word == ['q']:
				break
			word = self.players.players[0].wordTree.getWord(word)
			if word==None:
				print(colorama.Back.RED + 'BAD' + colorama.Style.RESET_ALL)
			else:
				print(colorama.Back.GREEN + 'GOOD' + colorama.Style.RESET_ALL)
			


	def startAutomaton(self):
		for x in range(15):
			for y in range(15):
				if self.tiles[x,y].letter != None:
					self.setOfLetters.remove(self.tiles[x,y].letter)
		self.players.pickLetters()
		while not self.players.done():
			self.print()
			self.playOneTurn()
		if self.players.finisher!=None:
			bonus = 0
			for p in self.players.players:
				temp = sum(constants.points[c] for c in p.set)
				p.score -= temp
				bonus += temp
			self.players.finisher.score += bonus
		self.print()
		print('total score : ',sum([pl.score for pl in self.players.players]))
	
	def playOneTurn(self):
		self.players.playOneTurn(self.show)

	def print(self):
		for x in range(15):
			str = ''.join([self.tiles[x,y].letter+" " if self.tiles[x,y].letter!=None else "_ " for y in range(15)])
			print(str)
		print()
		for player in self.players.players:
			print(player.getStatus())
		print('lettres left:',len(self.setOfLetters))

	def isEmpty(self):
		for x in range(15):
			for y in range(15):
				if self.tiles[x,y].letter!=None:
					return False
		return True

	def getAllWords(self):
		for _ in range(2):
			for x in range(15):
				for y in range(15):
					if self.tiles[x,y].letter!=None:
						if (x==14 or self.tiles[x+1,y].letter!=None) and (x==0 or self.tiles[x-1,y].letter==None):
							i=x; word='';
							while i<15 and self.tiles[i,y].letter!=None:
								word += self.tiles[i,y].letter
								i+=1
							yield word
			self.tiles = self.tiles.transpose()
			
	def checkAllWords(self):
		allWordsValid = True
		for word in self.getAllWords():
			if len(word)==1:
				continue
			if self.players.wordTree.getWord(word)==None:
				print("not valid:",word)
				allWordsValid = False
		return allWordsValid

	def invalidWords(self):
		unexpectedWords = []
		for word in self.getAllWords():
			if len(word)==1:
				continue
			if self.players.wordTree.getWord(word)==None:
				print("not valid:",word)
				unexpectedWords.append(word)
		return unexpectedWords

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
		return sum(self.getWordScore(w) for w in self.allWordsFormed(word)) + 50*int(word.numTrueLetters(self)==7)

	def wordExists(self, word):
		if self.players.wordTree.getWord(str(word))==None:
			return False
		else:
			return True

	def allWordsExist(self, word):
		for w in self.allWordsFormed(word):
			if self.wordExists(w)==False:
				return False
		return True

	def isValidMove(self, word):
		'''Does not check for words validity or board coherence. 
		Only checks the principle that your word must interact with existing letters.'''

		if self.isEmpty(): # First move must pass through middle
			passThroughMiddle = False
			for x,y,_ in word.getLetters():
				if x==7 and y==7:
					passThroughMiddle = True
			if passThroughMiddle:
				return True
			else:
				return False
		for x,y,_ in word.getLetters(): # word object must contain whole word
			pass
		if x+word.xI<15 and y+(1-word.xI)<15:
			if self.tiles[x+word.xI,y+(1-word.xI)].letter!=None:
				return False
		if len([w for w in self.allWordsFormed(word)])>1: # maybe other words formed
			return True
		else:
			playsAtLeastOneTile = False # if not, must use another letter of the board, and use one new letter
			usesExistingTile = False
			for x,y,_ in word.getLetters():
				if self.tiles[x,y].letter==None:
					playsAtLeastOneTile = True
				else:
					usesExistingTile = True
			if usesExistingTile and playsAtLeastOneTile:
				return True
			else:
				return False

	def play(self, word, set):
		if not self.isValidMove(word):
			raise WordError("word is not a valid move")
		if not self.allWordsExist(word):
			raise WordError("some words formed do not exist")
		score = self.getScore(word.replaceJoker(set, self))
		for x,y,c in word.getLetters():
			try:
				if self.tiles[x,y].playTile(c):
					try:
						set.remove(c)
					except ValueError:
						set.remove('0')
						c = '0'
			except TileError:
				self.print()
				print("Error while playing",word,"at",word.x,word.y,"horizontal",word.horizontal)
				raise
		self.log.append(word)
		return score

	def getNewLetters(self, nNewLetters):
		newLetters = []
		if nNewLetters>=len(self.setOfLetters):
			newLetters, self.setOfLetters = self.setOfLetters, newLetters
		else:
			shuffle(self.setOfLetters)
			newLetters = [self.setOfLetters.pop() for _ in range(nNewLetters)]
		return newLetters
