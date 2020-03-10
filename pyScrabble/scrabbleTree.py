import sys

from bisect import bisect_left

class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

    def word(word, indent=""):
    	if word.isWord:
    		str = bcolors.OKGREEN
    	else:
    		str = bcolors.WARNING
    	str += indent + word.data + bcolors.ENDC
    	return str

class KeyifyList(object):
	'''wrapper class to be able to use bisort functions with key functions'''
	def __init__(self, inner, key):
	    self.inner = inner
	    self.key = key

	def __len__(self):
		return len(self.inner)

	def __getitem__(self, k):
		return self.key(self.inner[k])







###################################################################################################################

# MAIN CLASS FOR SCRABBLE DATA BASE

class TreeNode(object):
	'''Recursive data structure. Stores words letter by letter as a tree structure.
	All methods are meant to be called from top tree. They are not called from subnodes other than for recursion (except print for debug reasons).'''
	def __init__(self, data="", isWord=True):
		self.data = data
		self.isWord = isWord
		self.children = []
		self.parent = None

	def getRoot(self):
		node = self
		while node.parent != None:
			node = node.parent
		return node


	def addChild(self, child):
		idx = bisect_left(KeyifyList(self.children, lambda c: c.data), child.data)
		child.parent = self
		self.children.insert(idx, child)
	def addNewChild(self, *args, **kwargs):
		temp = TreeNode(*args, **kwargs)
		self.addChild(temp)
		return temp

	def addWord(self, word):
		if len(word) == 1:
			self.addNewChild(word, isWord=True)
			return
		nextBranch = self.getNextBranch(word[0])
		if nextBranch==None:
			nextBranch = self.addNewChild(word[0], isWord=False)
		nextBranch.addWord(word[1:])

	def getNextBranch(self, c):
		idx = bisect_left(KeyifyList(self.children, lambda c: c.data), c)
		if idx>=len(self.children) or c != self.children[idx].data:
			return None
		else:
			return self.children[idx]

	def getNode(self, word):
		nb = self.getNextBranch(word[0])
		if len(word)==1 or nb==None:
			return nb
		else:
			return nb.getNode(word[1:])	
	def getWord(self, word):
		'''same as getNode but returns None if the node is not a word'''
		node = self.getNode(word)
		if node==None or not node.isWord:
			return None
		else:
			return node

	### Scrabble methodes ###
	### The variable 'set' stands for the set of letters that the player wants to play, plus maybe other letters from the board.

	def getAllPermutations(self, set, prefix=[]):
		'''Not used in practice (because we use a tree structure). We keep it here for reference'''
		myList = list(set)
		if len(myList)==1:
			yield prefix + myList
		else:
			for idx in range(len(myList)):
				yield from self.getAllPermutations(myList[:idx]+myList[idx+1:], prefix=prefix+[myList[idx]])

	def getAllAnagrams(self, set):
		'''Mix between permutations algo and breadth-first tree search'''
		setList = list(set)
		if len(setList)==0:
			if self.isWord:
				yield self
			else:
				return
		for idx in range(len(setList)):
			if setList[idx]=="0":
				for child in self.children:
					yield from child.getAllAnagrams(setList[:idx]+setList[idx+1:])
			else:
				node = self.getNextBranch(setList[idx])
				if node==None:
					continue
				yield from node.getAllAnagrams(setList[:idx]+setList[idx+1:])

	def getSubsetIndices(self, nLettersTotal, nLettersToTake, currentIndices=[]):
		'''take nLettersToTake among nLettersTotal, as indices, 
			without repetition or permutation'''
		if nLettersToTake >= nLettersTotal:
			yield [n for n in range(nLettersTotal)]
			return
		if len(currentIndices)==nLettersToTake:
			yield currentIndices
			return
		min = 0 if len(currentIndices)==0 else currentIndices[-1]+1
		for idx in range(min, nLettersTotal):
			yield from self.getSubsetIndices(nLettersTotal, nLettersToTake, currentIndices=currentIndices+[idx])

	def getSubset(self, set, nLetters):
		'''generates sets of nLetters taken from set'''
		for indices in self.getSubsetIndices(len(set), nLetters):
			yield ''.join([set[n] for n in indices])

	def getWordsFrom(self, set):
		'''Return all playable words with letters from set'''
		for nLetters in range(2, len(set)+1):
			for subset in self.getSubset(set, nLetters):
				yield from self.getAllAnagrams(subset)

	### End of scrabble methods ###



	### Utils
	def asString(self):
		prefix=""
		node = self.parent
		while not node==None:
			prefix = node.data + prefix
			node = node.parent
		return (prefix + self.data)
	def print(self, indent="", onlyWords=False, getParent=True):
		### if first call of recursion, get previous letters in tree
		if getParent==True:
			node = self.parent
			while not node==None:
				indent = node.data + indent
				node = node.parent
		### print current node
		if not onlyWords or self.isWord:
			print(bcolors.word(self, indent=indent))
		### print children nodes
		for child in self.children:
			child.print("  "+indent+self.data, onlyWords=onlyWords, getParent=False)
	def save(self, path):
		import pickle
		print('saving word tree')
		with open(path, "wb") as f:
			pickle.dump(self, f)



### END OF CLASS
############################################################################################



def createTree(path):
	with open(path, 'r') as f:
		words = f.read().splitlines()

	scrabbleTree = TreeNode('', isWord=False)
	for word in words:
		sys.stdout.write(word + " "*10)
		sys.stdout.write("\r")
		sys.stdout.flush()
		scrabbleTree.addWord(word)

	print()
	return scrabbleTree

def loadTree(path):
	import pickle
	print("loading word tree")
	with open(path, "rb") as f:
		tree = pickle.load(f)
	return tree