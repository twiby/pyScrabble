import os
import sys
import argparse

import pyScrabble.scrabbleBoard as sb


def main(args):
	if args.scrabbleStats:
		sb.computeScrabbleStats()
		sys.exit(0)

	if args.checkWords:
		board = sb.Board()
		print({w.asString() for w in board.players.wordTree.getAllAnagrams(args.checkWords)})
		sys.exit(0)

	if args.players:
		nPlayers = args.players
	else:
		nPlayers = 1

	if args.auto:
		board = sb.Board(nPlayers=nPlayers)
		board.startAutomaton()
		sys.exit(0)

	else:
		board = sb.Board()
		board.startAdviser()


if __name__=="__main__":
	parser = argparse.ArgumentParser(description="Automatons to play scrabble")
	parser.add_argument('--checkWords', type=str, help="returns all words formed with these letters")
	parser.add_argument('--players', type=int, help="number of players")
	parser.add_argument('--scrabbleStats', action='store_true', help="makes stats about probability of having a scrabble")
	parser.add_argument('--auto', action='store_true', help="launches a game with automatic players")
	args = parser.parse_args()
	main(args)