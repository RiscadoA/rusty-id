mod id;
mod name;
mod registry;

pub use id::Id;
pub use name::{Name, is_valid_name_segment};
pub use registry::{PartialRegistry, Registry};

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct MyId(u8);

    impl Id for MyId {
        fn from_index(index: usize) -> Self {
            MyId(index as u8)
        }

        fn to_index(self) -> usize {
            self.0 as usize
        }
    }

    #[test]
    fn test_is_valid_name_segment() {
        assert!(name::is_valid_name_segment("valid_name"));
        assert!(!name::is_valid_name_segment(""));
        assert!(!name::is_valid_name_segment("Invalid"));
        assert!(!name::is_valid_name_segment("invalid-name"));
        assert!(!name::is_valid_name_segment("foo:bar"));
    }

    #[test]
    fn test_name_creation() {
        assert!(Name::new("scope:name").is_some());
        assert!(Name::new("scope:name_bar").is_some());
        assert!(Name::new("scope:name-bar").is_none());
        assert!(Name::new("scope:").is_none());
        assert!(Name::new(":name").is_none());
        assert!(Name::new("scope:name:extra").is_none());
        assert!(Name::new("scope:name!").is_none());
    }

    #[test]
    fn test_registry() {
        let mut registry = Registry::<MyId, bool>::new();

        assert_eq!(registry.len(), 0);
        assert_eq!(registry.add_anonymous(true), MyId(0));
        assert_eq!(registry.len(), 1);
        assert!(*registry.get(MyId(0)) == true);
        assert_eq!(registry.add_named("test:name".into(), false), Ok(MyId(1)));
        assert_eq!(registry.len(), 2);
        assert_eq!(registry.find(&"test:name".into()), Some(MyId(1)));
        assert!(!registry.contains(&"test:other".into()));
        assert!(registry.contains(&"test:name".into()));
        assert_eq!(*registry.get(MyId(1)), false);
        assert!(registry.add_named("test:name".into(), true).is_err());
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn test_partial_registry() {
        let mut partial_registry = PartialRegistry::<MyId>::new();

        assert_eq!(partial_registry.add_anonymous(), MyId(0));
        assert_eq!(partial_registry.add_named("test:name".into()), Ok(MyId(1)));
        assert_eq!(partial_registry.find(&"test:name".into()), Some(MyId(1)));
        assert!(!partial_registry.contains(&"test:other".into()));
        assert!(partial_registry.contains(&"test:name".into()));
        assert!(partial_registry.add_named("test:name".into()).is_err());
        assert_eq!(partial_registry.len(), 2);

        let registry = partial_registry.build(|k| k.0);
        assert_eq!(*registry.get(MyId(0)), 0);
        assert_eq!(*registry.get(MyId(1)), 1);
        assert_eq!(registry.len(), 2);
    }
}
