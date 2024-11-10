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
> library(jellyfishr)
> hamming_distance("00100", "00000")
[1] 1
> levenshtein_distance('jellyfishr', 'jellyfisher')
[1] 1
> jaro_distance('jellyfishr', 'jellyfisher')
[1] 0.03030303
> jaro_winkler_distance('jellyfishr', 'jellyfisher')
[1] 0.01818182
> damerau_levenshtein_distance('fisherman', 'ifsherman')
[1] 1
> jaccard_distance('John Smith', c('John Jacob Smith', 'Smith John'), ngram_size = NULL)
[1] 0.3333333 0.0000000
> soundex("Jellyfish")
[1] "J412"
> metaphone("Jellyfish")
[1] "JLFX"
> nysiis("Jellyfish")
[1] "JALYF"
```

All functions are vectorized:

```R
> levenshtein_distance(c('jellyfishr', 'jellyfishr'), c('jellyfisher', 'jollyfisher'))
[1] 1 2
```

Shorter vectors are recycled:

```R
> levenshtein_distance(c('jellyfishr', 'jellyfisher'), 'jellyfisher')
[1] 1 0
```

`NA`s produce `NA` outputs:

```R
> levenshtein_distance(c('jellyfishr', NA), 'jellyfisher')
[1] 1 NA
```

## Build

```R
rextendr::document()
devtools::load_all()
```
