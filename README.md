# jellyfishr

**jellyfishr** is a port of [jellyfish](https://github.com/jamesturk/jellyfish), a Python library for approximate & phonetic matching of strings, to R.

Source: [https://github.com/NiklasvonM/jellyfishr](https://github.com/NiklasvonM/jellyfishr)

## Included Algorithms

String comparison:

* Damerau-Levenshtein distance
* Hamming distance
* Jaccard distance
* Jaro distance
* Jaro-Winkler distance
* Levenshtein distance

Phonetic encoding:

* American Soundex
* Metaphone
* NYSIIS (New York State Identification and Intelligence System)

## Example Usage

```R
>>> library(jellyfishr)
>>> levenshtein_distance(c('jellyfishr', 'jellyfishr'), c('jellyfisher', 'jollyfisher'))
[1] 1 2
>>> jaro_distance('jellyfishr', 'jellyfisher')
[1] 0.03030303
>>> jaro_winkler_distance('jellyfishr', 'jellyfisher')
[1] 0.01818182
>>> damerau_levenshtein_distance('fisherman', 'ifsherman')
[1] 1
>>> jaccard_distance('John Smith', c('John Jacob Smith', 'Smith John'), ngram_size = NULL)
[1] 0.3333333 0.0000000
```

## Build

```R
rextendr::document()
devtools::load_all()
```
