import os
import sys

### load other file
class add_path():
    def __init__(self, path):
        self.path = path

    def __enter__(self):
        for i in range(len(self.path)):
            sys.path.insert(0, self.path[i])

    def __exit__(self, exc_type, exc_value, traceback):
        for i in range(len(self.path)):
            sys.path.remove(self.path[i])

folder = os.path.dirname(os.path.realpath(__file__)) + "/"
python_sys_path = [folder[:-1]]
with add_path(python_sys_path):
    utils = __import__("scrabbleTreeUtils")


### Get all the words of scrabble in a text file
if not os.path.isfile(folder + "scrabbleWords.txt"):
	utils.writeScrabbleWordsToNewFile(folder + "scrabbleWords.txt")

### Organize the data in a tree
if not os.path.isfile(folder + "scrabble.tree"):
	scrabbleTree = utils.createTree(folder + "scrabbleWords.txt")
	scrabbleTree.save(folder + "scrabble.tree")
else:
	scrabbleTree = utils.loadTree(folder + "scrabble.tree")

# scrabbleTree.children[0].children[-1].print()
# scrabbleTree.children[0].children[-1].children[0].print()
# print(scrabbleTree.getWord("azure").asString())
# TWELLOS


print({x.asString() for x in scrabbleTree.getAllAnagrams("apporte")})
