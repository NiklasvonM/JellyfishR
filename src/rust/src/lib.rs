mod common;
mod hamming;
mod jaro;
mod levenshtein;

//use crate::hamming::hamming_distance;
use crate::hamming::hamming_distance_;
use crate::jaro::{jaro_similarity_, jaro_winkler_similarity_};
use crate::levenshtein::{damerau_levenshtein_distance_, levenshtein_distance_};
use extendr_api::prelude::*;

// Generic macro to turn scalar functions into vectorized ones.
macro_rules! make_vector_fn {
    ($fn_name:ident, $vec_fn_name:ident, $ret_type:ty, $convert_fn:expr) => {
        #[extendr]
        /// @export
        fn $vec_fn_name(s1: Strings, s2: Strings) -> Result<Vec<$ret_type>> {
            let mut result: Vec<$ret_type> = Vec::new();
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
            let shorter_repeated = shorter_vec.iter().cycle().take(longer.len());

            for (elem1, elem2) in longer.iter().zip(shorter_repeated) {
                result.push(if elem1.is_na() || elem2.is_na() {
                    <$ret_type>::na()
                } else {
                    $convert_fn($fn_name(&elem1.as_str(), &*elem2))
                });
            }
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

// Now we can use the generic macro for both integer and float functions
make_vector_fn!(
    damerau_levenshtein_distance_,
    damerau_levenshtein_distance,
    Rint,
    to_rint
);
make_vector_fn!(jaro_similarity_, jaro_similarity, Rfloat, to_rfloat);
make_vector_fn!(
    jaro_winkler_similarity_,
    jaro_winkler_similarity,
    Rfloat,
    to_rfloat
);
make_vector_fn!(hamming_distance_, hamming_distance, Rint, to_rint);
make_vector_fn!(levenshtein_distance_, levenshtein_distance, Rint, to_rint);

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod jellyfishr;
    fn damerau_levenshtein_distance;
    fn hamming_distance;
    fn jaro_similarity;
    fn jaro_winkler_similarity;
    fn levenshtein_distance;
}
