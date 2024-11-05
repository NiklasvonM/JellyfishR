# jellyfishr

**jellyfishr** is a port of [jellyfish](https://github.com/jamesturk/jellyfish), a Python library for approximate & phonetic matching of strings, to R.

Source: [https://github.com/NiklasvonM/jellyfishr](https://github.com/NiklasvonM/jellyfishr)

## Included Algorithms

String comparison:

* Damerau-Levenshtein Distance
* Hamming Distance
* Jaccard Index
* Jaro Similarity
* Jaro-Winkler Similarity
* Levenshtein Distance

## Example Usage

```R
>>> library(jellyfishr)
>>> levenshtein_distance(c('jellyfishr', 'jellyfishr'), c('jellyfisher', 'jollyfisher'))
[1] 1 2
>>> jaro_similarity('jellyfishr', 'jellyfisher')
[1] 0.969697
>>> jaro_winkler_similarity('jellyfishr', 'jellyfisher')
[1] 0.9818182
>>> damerau_levenshtein_distance('fisherman', 'ifsherman')
[1] 1
```

## Build

```R
rextendr::document()
devtools::load_all()
```
