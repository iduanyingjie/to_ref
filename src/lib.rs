pub trait ToRef {
    fn to_ref<T>(&self) -> &T
    where
        T: ?Sized,
        Self: AsRef<T>,
    {
        self.as_ref()
    }
}

impl<T> ToRef for T {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::ToRef;
        use std::path::Path;
        let url = "www.google.com".to_string();
        let path = url.to_ref::<Path>();
        assert_eq!(path, AsRef::<Path>::as_ref("www.google.com"));
    }
}
