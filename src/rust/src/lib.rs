mod common;
mod hamming;
mod jaccard;
mod jaro;
mod levenshtein;
mod metaphone;
mod nysiis;
mod soundex;

//use crate::hamming::hamming_distance;
use crate::hamming::hamming_distance_;
use crate::jaccard::jaccard_distance_;
use crate::jaro::{jaro_distance_, jaro_winkler_distance_};
use crate::levenshtein::{damerau_levenshtein_distance_, levenshtein_distance_};
use crate::metaphone::metaphone_;
use crate::nysiis::nysiis_;
use crate::soundex::soundex_;

use extendr_api::prelude::*;
use rayon::prelude::*;

// Macro to turn 1-ary scalar functions into vectorized ones.
macro_rules! vectorize_arity_one {
    ($fn_name:ident, $vec_fn_name:ident, $ret_type:ty, $convert_fn:expr) => {
        #[extendr]
        /// @export
        fn $vec_fn_name(s: Strings) -> Result<Vec<$ret_type>> {
            let s: Vec<&str> = s.iter().map(|rstr| rstr.as_str()).collect();

            let result: Vec<$ret_type> = s
                .iter()
                .map(|elem| {
                    if elem.is_na() {
                        <$ret_type>::na()
                    } else {
                        $convert_fn($fn_name(elem))
                    }
                })
                .collect();

            Ok(result)
        }
    };
}

// Macro to turn 2-ary scalar functions into vectorized ones.
macro_rules! vectorize_arity_two {
    ($fn_name:ident, $vec_fn_name:ident, $ret_type:ty, $convert_fn:expr) => {
        #[extendr]
        /// @export
        fn $vec_fn_name(s1: Strings, s2: Strings) -> Result<Vec<$ret_type>> {
            let (longer, shorter) = if s1.len() > s2.len() {
                (s1, s2)
            } else {
                (s2, s1)
            };

            if shorter.len() == 0 {
                return Err("One of the string vectors is not given!".into());
            }

            if longer.len() % shorter.len() != 0 {
                return Err(
                    "Longer vector length must be a multiple of shorter vector length".into(),
                );
            }

            let shorter_vec: Vec<&str> = shorter.iter().map(|rstr| rstr.as_str()).collect();
            let shorter_repeated: Vec<&str> = shorter_vec
                .iter()
                .cycle()
                .take(longer.len())
                .map(|s| *s)
                .collect();
            let longer_vec: Vec<&str> = longer.iter().map(|rstr| rstr.as_str()).collect();

            let result: Vec<$ret_type> = longer_vec
                .iter()
                .zip(shorter_repeated.iter())
                .collect::<Vec<_>>()
                .into_par_iter()
                .map(|(elem1, elem2)| {
                    if elem1.is_na() || elem2.is_na() {
                        <$ret_type>::na()
                    } else {
                        $convert_fn($fn_name(&elem1, &elem2))
                    }
                })
                .collect();

            Ok(result)
        }
    };
}

macro_rules! vectorize_jaccard {
    ($fn_name:ident, $vec_fn_name:ident, $ret_type:ty, $convert_fn:expr) => {
        #[extendr]
        /// @export
        fn $vec_fn_name(
            s1: Strings,
            s2: Strings,
            ngram_size: Option<usize>,
        ) -> Result<Vec<$ret_type>> {
            let (longer, shorter) = if s1.len() > s2.len() {
                (s1, s2)
            } else {
                (s2, s1)
            };

            if shorter.len() == 0 {
                return Err("One of the string vectors is not given!".into());
            }

            if longer.len() % shorter.len() != 0 {
                return Err(
                    "Longer vector length must be a multiple of shorter vector length".into(),
                );
            }

            let shorter_vec: Vec<&str> = shorter.iter().map(|rstr| rstr.as_str()).collect();
            let shorter_repeated: Vec<&str> = shorter_vec
                .iter()
                .cycle()
                .take(longer.len())
                .map(|s| *s)
                .collect();
            let longer_vec: Vec<&str> = longer.iter().map(|rstr| rstr.as_str()).collect();

            let result: Vec<$ret_type> = longer_vec
                .iter()
                .zip(shorter_repeated.iter())
                .collect::<Vec<_>>()
                .into_par_iter()
                .map(|(elem1, elem2)| {
                    if elem1.is_na() || elem2.is_na() {
                        <$ret_type>::na()
                    } else {
                        $convert_fn($fn_name(&elem1, &elem2, ngram_size))
                    }
                })
                .collect();

            Ok(result)
        }
    };
}

fn to_rint(val: usize) -> Rint {
    Rint::from(val as i32)
}

fn to_rfloat(val: f64) -> Rfloat {
    Rfloat::from(val)
}

fn to_rstr(val: String) -> Rstr {
    Rstr::from(val)
}

vectorize_arity_one!(metaphone_, metaphone, Rstr, to_rstr);
vectorize_arity_one!(soundex_, soundex, Rstr, to_rstr);
vectorize_arity_one!(nysiis_, nysiis, Rstr, to_rstr);

vectorize_arity_two!(
    damerau_levenshtein_distance_,
    damerau_levenshtein_distance,
    Rint,
    to_rint
);
vectorize_arity_two!(jaro_distance_, jaro_distance, Rfloat, to_rfloat);
vectorize_arity_two!(
    jaro_winkler_distance_,
    jaro_winkler_distance,
    Rfloat,
    to_rfloat
);
vectorize_arity_two!(hamming_distance_, hamming_distance, Rint, to_rint);
vectorize_arity_two!(levenshtein_distance_, levenshtein_distance, Rint, to_rint);

vectorize_jaccard!(jaccard_distance_, jaccard_distance, Rfloat, to_rfloat);

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod jellyfishr;

    fn damerau_levenshtein_distance;
    fn hamming_distance;
    fn jaccard_distance;
    fn jaro_distance;
    fn jaro_winkler_distance;
    fn levenshtein_distance;
    fn metaphone;
    fn nysiis;
    fn soundex;
}
