# Installation
pyScrabble depends on code which is written in Rust, so be sure to install Rust at https://www.rust-lang.org/.
You can install all dependencies and compile the Rust code with the single command:
```
pip install .
```

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

