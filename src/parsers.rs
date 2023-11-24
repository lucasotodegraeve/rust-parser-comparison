mod nom;
mod pest;
mod vanilla;

pub use nom::parse as parse_nom;
pub use nom::parse_simple as parse_simple_nom;

pub use pest::parse as parse_pest;
pub use pest::parse_simple as parse_simple_pest;

pub use vanilla::parse as parse_vanilla;
pub use vanilla::parse_vanilla_slow;
