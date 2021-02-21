use data::link::Link;

/// Linkstream Iterator type. Alias for Iterator<Item=Link>
pub type LinkIterator = dyn Iterator<Item=Link>;
