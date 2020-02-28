import numpy as np

class Board(object):
	def __init__(self):
		self.wordFactorGrid = np.array([
			[3, 1, 1, 1, 1, 1, 1, 3],
			[1, 2, 1, 1, 1, 1, 1, 1],
			[1, 1, 2, 1, 1, 1, 1, 1],
			[1, 1, 1, 2, 1, 1, 1, 1],
			[1, 1, 1, 1, 2, 1, 1, 1],
			[1, 1, 1, 1, 1, 1, 1, 1],
			[1, 1, 1, 1, 1, 1, 1, 1],
			[3, 1, 1, 1, 1, 1, 1, 2]])
		self.letterFactorGrid = np.array([
			[1, 1, 1, 2, 1, 1, 1, 1],
			[1, 1, 1, 1, 1, 3, 1, 1],
			[1, 1, 1, 1, 1, 1, 2, 1],
			[2, 1, 1, 1, 1, 1, 1, 2],
			[1, 1, 1, 1, 1, 1, 1, 1],
			[1, 3, 1, 1, 1, 3, 1, 1],
			[1, 1, 2, 1, 1, 1, 2, 1],
			[1, 1, 1, 2, 1, 1, 1, 1]])
		self.wordFactorGrid = np.pad(self.wordFactorGrid, ((0, 7),(0, 7)), 'reflect')
		self.letterFactorGrid = np.pad(self.letterFactorGrid, ((0, 7),(0, 7)), 'reflect')
