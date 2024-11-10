rextendr::document()
devtools::load_all()
library(jellyfishr)
library(stringdist)
library(microbenchmark)

N <- 1000
TIMES <- 10

# Generate long random strings
set.seed(123)
str1 <- sapply(seq_len(N), function(x)
  paste(sample(letters, 100, replace = TRUE), collapse = ""))
str2 <- sapply(seq_len(N), function(x)
  paste(sample(letters, 100, replace = TRUE), collapse = ""))

assertthat::are_equal(levenshtein_distance(str1, str2),
                      stringdist(str1, str2, method = "lv"))
assertthat::are_equal(hamming_distance(str1, str2),
                      stringdist(str1, str2, method = "hamming"))
assertthat::are_equal(jaro_winkler_distance(str1, str2),
                      stringdist(str1, str2, method = "jw"))
assertthat::are_equal(jaccard_distance(str1, str2, 1),
                      stringdist(str1, str2, method = "jaccard", q = 1))
assertthat::are_equal(jaccard_distance(str1, str2, 5),
                      stringdist(str1, str2, method = "jaccard", q = 5))


microbenchmark(
  jellyfishr = levenshtein_distance(str1, str2),
  stringdist = stringdist(str1, str2, method = "lv"),
  times = TIMES
)

# Benchmark Hamming distance
microbenchmark(
  jellyfishr = hamming_distance(str1, str2),
  stringdist = stringdist(str1, str2, method = "hamming"),
  times = TIMES
)

microbenchmark(
  jellyfishr = jaro_winkler_distance(str1, str2),
  stringdist = stringdist(str1, str2, method = "jw"),
  times = TIMES
)

microbenchmark(
  jellyfishr = jaccard_distance(str1, str2, 1),
  stringdist = stringdist(str1, str2, method = "jaccard", q = 1),
  times = TIMES
)

microbenchmark(
  jellyfishr = jaccard_distance(str1, str2, 5),
  stringdist = stringdist(str1, str2, method = "jaccard", q = 5),
  times = TIMES
)

microbenchmark(
  jellyfishr = jaccard_distance(str1, str2, 10),
  stringdist = stringdist(str1, str2, method = "jaccard", q = 10),
  times = TIMES
)
