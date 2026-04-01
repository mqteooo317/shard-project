use std::fmt;

impl fmt::Display for CacheKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)?;
        if let Some(ref frag) = self.fragment_id {
            write!(f, "#{}", frag)?;
        }
        if let Some(ref variant) = self.variant {
            write!(f, "@{}", variant)?;
        }
        Ok(())
    }
}