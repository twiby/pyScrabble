import numpy as np
from pyScrabble import constants as c

class Tile(object):
	def __init__(self, wordFactor, letterFactor, letter=None):
		self.wordFactor = wordFactor
		self.letterFactor = letterFactor
		self.letter = letter

class Board(object):
	def __init__(self):
		wordFactorGrid = np.pad(c.wordFactorGrid, ((0, 7),(0, 7)), 'reflect')
		letterFactorGrid = np.pad(c.letterFactorGrid, ((0, 7),(0, 7)), 'reflect')
		self.tiles = [[Tile(wordFactorGrid[x,y], letterFactorGrid[x,y]) for x in range(15)] for y in range(15)]
		self.setOfLetters = c.setOfLetters
		
	def print(self):
		for x in range(15):
			print([self.tiles[x][y].letter if self.tiles[x][y].letter!=None else " " for y in range(15)])