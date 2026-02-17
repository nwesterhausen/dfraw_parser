//! String token to parsed tag map for tile page tokens.

use crate::tokens::TilePageToken;

/// Map of tile page tags to their string representation.
pub static TILE_PAGE_TOKENS: phf::Map<&'static str, TilePageToken> = phf::phf_map! {
    "TILE_DIM" => TilePageToken::TileDim,
    "PAGE_DIM_PIXELS" => TilePageToken::PageDim,
    "PAGE_DIM" => TilePageToken::PageDim,
    "FILE" => TilePageToken::File,
};
