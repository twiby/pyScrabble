from lxml import html
import requests
import sys



### Utils to get words from internet
def replaceWeirdC(string):
	for i in range(len(string)):
		if string[i]=="ç":
			string = string[:i]+"%C3%A7"+string[i+1:]
	return string


def writeScrabbleWordsToNewFileOld(path):
	homePageLink = "http://scrabblemania.fr"
	page = requests.get(homePageLink+"/tous-les-mots")
	tree = html.fromstring(page.content)

	allLinks = tree.xpath('//a')
	selectedLinks = []
	for link in allLinks:
		string = link.attrib['href']
		if string[:9]=="/mots-de-" and string[-8:]!="scrabble":
			selectedLinks.append(homePageLink+ replaceWeirdC(string))
		else:
			continue


	chars = set('0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ/ \\')
	with open(path, 'w') as f:

		for link in selectedLinks:
			sys.stdout.write(link+" "*10)
			sys.stdout.write('\r')
			sys.stdout.flush()

			page = requests.get(link)
			tree = html.fromstring(page.content)
			textContents = tree.xpath('//a/text()')[8:-9]
			n = 0
			for txt in textContents:
				if any((c in chars) for c in txt):
					n+=1
				else:
					f.write("%s\n" % txt)
			if n==0:
				continue
			otherLinks = tree.xpath('//a')[8:-9]
			otherLinks = otherLinks[-n:]
			for link in otherLinks:
				sys.stdout.write(homePageLink+replaceWeirdC(link.attrib['href'])+" "*10)
				sys.stdout.write('\r')
				sys.stdout.flush()
				page = requests.get(homePageLink+replaceWeirdC(link.attrib['href']))
				tree = html.fromstring(page.content)
				textContents = tree.xpath('//a/text()')[8:-9]
				textContents = textContents[:-n]
				for txt in textContents:
					f.write("%s\n" % txt)
	print()

def writeScrabbleWordsToNewFile(path):
	with open(path, 'w') as f:

		page = requests.get('https://www.listesdemots.net/touslesmots.htm')
		tree = html.fromstring(page.content)
		for word in tree.xpath('//span/text()')[1].split(' '):
			f.write("%s\n" % word.lower())

		for page in range(2,919):
			sys.stdout.write("page "+str(page))
			sys.stdout.write("\r")
			sys.stdout.flush()

			page = requests.get('https://www.listesdemots.net/touslesmotspage'+str(page)+'.htm')
			tree = html.fromstring(page.content)
			for word in tree.xpath('//span/text()')[1].split(' '):
				f.write("%s\n" % word.lower())
		print()
