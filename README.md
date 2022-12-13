# Installation
pyScrabble depends on a git submodule which is written in Rust, so be sure to install Rust at https://www.rust-lang.org/.
To pull the submodule:
```
git submodule init
git pull --recurse-submodules
```

As usual to install python modules, run in a venv:
```
pip install -r requirements
```

Additionnally, to compile and install as a python package the submodule run the following
```
cd rusted_tree
maturin build -r
pip install target/wheels/<name_of_the_produced_wheel.whl>
```
You can check that a new module is now available via calling `pip freeze`


# pyScrabble
a utility for scrabble.

pyScrabble will download words from the french ODS8 dictionnary and create our custom data structures, saving them both to files, if they are missing. 
Default usage is an interface where you can fill in the board and your letters and it gives you the best play.
Auto usage, for testing and fun, is
```
./pyScrabble --auto
```
for help on the different options, use
```
./pyScrabble -h
```


# Personalisation

For those who want to try their own algorithms, data structures and utils are defined in scrableTree.py 
(getWord and getAllAnagrams are the main functions used outside)

The algo for searching best move is in pyScrabbleUtils/players.py in the method Player.findBestWord
