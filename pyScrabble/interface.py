import pyScrabble.scrabbleBoard as sb
import pyScrabble.constants as c
import sys,tty,termios

wordFactorColors   = dict(((1,''),(2,'\033[33m'),(3,'\033[31m')))
letterFactorColors = dict(((1,''),(2,'\033[36m'),(3,'\033[34m')))

class _Getch:       
	def __call__(self):
		fd = sys.stdin.fileno()
		old_settings = termios.tcgetattr(fd)
		try:
			tty.setraw(sys.stdin.fileno())
			ch = sys.stdin.read(1)
		finally:
			termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
		return ch

def getSingleInput():
	inkey = _Getch()
	for _ in range(6):
		while(1):
			k=inkey()
			if k!='':
				break
		print(ord(k[0]))

def getInput():
	inkey = _Getch()
	escapeSignal=''
	escapeSignalLength=0
	while(1):
		k=inkey()
		if k!='' and k!='\x1b' and escapeSignalLength==0:
			break
		else:
			escapeSignal+=k
			escapeSignalLength+=1
			if escapeSignalLength==3:
				k=escapeSignal
				break
	return k

def getScrabbleBoard(board):
	print("Enter current board")
	for x in range(15):
		str = ''
		for y in range(15):
			str += wordFactorColors[board.tiles[x,y].wordFactor] + letterFactorColors[board.tiles[x,y].letterFactor]
			if board.tiles[x,y].letter==None:
				str += '_ '
			else:
				str += board.tiles[x,y].letter+' '
			str += '\033[0m'
		sys.stdout.write(str+'\n')
		sys.stdout.flush()
	sys.stdout.write("\033[A")
	sys.stdout.flush()
	x=14; y=0

	while(1):
		k = getInput()
		if k=='\x1b[A':
			if x!=0:
				x-=1
				sys.stdout.write("\033[A")
		elif k=='\x1b[B':
			if x!=14:
				x+=1
				sys.stdout.write("\033[B")
		elif k=='\x1b[C':
			if y!=14:
				y+=1
				sys.stdout.write("\033[C\033[C")
		elif k=='\x1b[D':
			if y!=0:
				y-=1
				sys.stdout.write("\033[D\033[D")
		elif k=='\r':
			break
		elif k=="\x7f":
			board.tiles[x,y].letter=None
			board.tiles[x,y].wordFactor = c.wordFactorGrid[x,y]
			board.tiles[x,y].letterFactor = c.letterFactorGrid[x,y]
			sys.stdout.write(wordFactorColors[c.wordFactorGrid[x,y]]+letterFactorColors[c.letterFactorGrid[x,y]]+"_"+"\033[0m"+"\033[D")
		else:
			board.tiles[x,y].letter=k
			board.tiles[x,y].wordFactor = 1
			board.tiles[x,y].letterFactor = 1
			sys.stdout.write(k+"\033[D")
		sys.stdout.flush()
	for _ in range(15-x):
		print()
	if not board.checkAllWords():
		print("not all words are valid.")

	# for x in range(15):
	# 	for y in range(15):
	# 		if board.tiles[x,y].letter==None:
	# 			board.tiles[x,y].wordFactor=c.wordFactorGrid[x,y]
	# 			board.tiles[x,y].letterFactor=c.letterFactorGrid[x,y]
	# 		else:
	# 			board.tiles[x,y].wordFactor=1
	# 			board.tiles[x,y].letterFactor=1

	return board
