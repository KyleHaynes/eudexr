use extendr_api::prelude::*;

extern crate alloc;

mod algorithm;
mod counter;
mod result;

pub mod nstr;
pub mod str;


mod algorithms {
    pub mod bag;
    pub mod cosine;
    pub mod damerau_levenshtein;
    pub mod entropy_ncd;
    pub mod hamming;
    pub mod jaccard;
    pub mod jaro;
    pub mod jaro_winkler;
    pub mod lcsseq;
    pub mod lcsstr;
    pub mod length;
    pub mod levenshtein;
    pub mod lig3;
    pub mod mlipns;
    pub mod overlap;
    pub mod prefix;
    pub mod ratcliff_obershelp;
    pub mod roberts;
    pub mod sift4_common;
    pub mod sift4_simple;
    pub mod smith_waterman;
    pub mod sorensen_dice;
    pub mod suffix;
    pub mod tversky;
    pub mod yujian_bo;
}

// pub use self::algorithm::Algorithm;
pub use self::algorithms::bag::Bag;
pub use self::algorithms::cosine::Cosine;
pub use self::algorithms::damerau_levenshtein::DamerauLevenshtein;
pub use self::algorithms::entropy_ncd::EntropyNCD;
pub use self::algorithms::hamming::Hamming;
pub use self::algorithms::jaccard::Jaccard;
pub use self::algorithms::jaro::Jaro;
pub use self::algorithms::jaro_winkler::JaroWinkler;
pub use self::algorithms::lcsseq::LCSSeq;
pub use self::algorithms::lcsstr::LCSStr;
pub use self::algorithms::length::Length;
pub use self::algorithms::levenshtein::Levenshtein;
pub use self::algorithms::lig3::LIG3;
pub use self::algorithms::mlipns::MLIPNS;
pub use self::algorithms::overlap::Overlap;
pub use self::algorithms::prefix::Prefix;
pub use self::algorithms::ratcliff_obershelp::RatcliffObershelp;
pub use self::algorithms::roberts::Roberts;
pub use self::algorithms::sift4_common::Sift4Common;
pub use self::algorithms::sift4_simple::Sift4Simple;
pub use self::algorithms::smith_waterman::SmithWaterman;
pub use self::algorithms::sorensen_dice::SorensenDice;
pub use self::algorithms::suffix::Suffix;
pub use self::algorithms::tversky::Tversky;
pub use self::algorithms::yujian_bo::YujianBo;
pub use self::result::Result;


extendr_module! {
    mod algorithms;
    use Cosine;
}