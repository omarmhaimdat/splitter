import rsplitter
import timeit

import wordninja

model = rsplitter.LanguageModel("/Users/omarmhaimdat/Documents/splitter/src/corpus.txt")
# model.split("Thequickbrownfoxjumpsoverthelazydog")

model_ninja = wordninja.LanguageModel("/Users/omarmhaimdat/Documents/splitter/src/corpus.txt.gz")
# model_ninja.split("Thequickbrownfoxjumpsoverthelazydog")
# model.split("Thequickbrownfoxjumpsoverthelazydog")

def f():
    model.split("imateapot")

def g():
    model_ninja.split("imateapot")

print(timeit.timeit(f, number=10000))
print(timeit.timeit(g, number=10000))