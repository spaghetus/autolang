# autolang

*this is by far the most complicated joke program i've ever made and i'm not sure how i feel about that*

A program for generating bad conlang relexes given two lists of symbols to map between.

## Usage

1. Get a long passage in the source language.
2. Bake the word frequencies by running `autolang-freqs -f passage.txt -t lang-freqs.csv`
   * Optionally, pass `--dictionary dict.txt` with a text file containing all of the words the program is allowed to recognize.
3. Write a CSV file containing all of the symbols the language is allowed to use, in the `text` column.
4. Bake the Huffman tree with `autolang-to-mapping lang-freqs.csv dict.csv lang.json`
5. Translate into the new language with `echo text-to-translate | autolang-translate lang.json`
6. Translate from the new language with `echo text-to-translate | autolang-translate lang.json -r`