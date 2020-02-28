import numpy as np

class Tile(object):
	def __init__(self, wordFactor, letterFactor, letter=None):
		self.wordFactor = wordFactor
		self.letterFactor = letterFactor
		self.letter = letter

class Board(object):
	def __init__(self):
		wordFactorGrid = np.array([
			[3, 1, 1, 1, 1, 1, 1, 3],
			[1, 2, 1, 1, 1, 1, 1, 1],
			[1, 1, 2, 1, 1, 1, 1, 1],
			[1, 1, 1, 2, 1, 1, 1, 1],
			[1, 1, 1, 1, 2, 1, 1, 1],
			[1, 1, 1, 1, 1, 1, 1, 1],
			[1, 1, 1, 1, 1, 1, 1, 1],
			[3, 1, 1, 1, 1, 1, 1, 2]])
		letterFactorGrid = np.array([
			[1, 1, 1, 2, 1, 1, 1, 1],
			[1, 1, 1, 1, 1, 3, 1, 1],
			[1, 1, 1, 1, 1, 1, 2, 1],
			[2, 1, 1, 1, 1, 1, 1, 2],
			[1, 1, 1, 1, 1, 1, 1, 1],
			[1, 3, 1, 1, 1, 3, 1, 1],
			[1, 1, 2, 1, 1, 1, 2, 1],
			[1, 1, 1, 2, 1, 1, 1, 1]])
		wordFactorGrid = np.pad(wordFactorGrid, ((0, 7),(0, 7)), 'reflect')
		letterFactorGrid = np.pad(letterFactorGrid, ((0, 7),(0, 7)), 'reflect')
		self.tiles = [[Tile(wordFactorGrid[x,y], letterFactorGrid[x,y]) for x in range(15)] for y in range(15)]
		
