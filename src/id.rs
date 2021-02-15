use std::fmt;
use std::marker::PhantomData;
pub struct Id<T: ?Sized, R: PartialEq + Clone + fmt::Display + fmt::Debug>(R, PhantomData<T>);

impl<T: ?Sized, R: PartialEq + Clone + fmt::Display + fmt::Debug> Id<T, R> {
    pub fn new(raw_id: impl Into<R>) -> Self {
        Self(raw_id.into(), PhantomData)
    }
    pub fn raw_id(&self) -> &R {
        &self.0
    }
}

impl<T: ?Sized, R: PartialEq + Clone + fmt::Display + fmt::Debug> PartialEq for Id<T, R> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: ?Sized, R: PartialEq + Clone + fmt::Display + fmt::Debug> Clone for Id<T, R> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<T: ?Sized, R: PartialEq + Clone + fmt::Display + fmt::Debug> fmt::Display for Id<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<T: ?Sized, R: PartialEq + Clone + fmt::Display + fmt::Debug> fmt::Debug for Id<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    struct IdTag;
    #[test_case("hoge","hoge" => true)]
    #[test_case("hfoo","hoge" => false)]
    #[test_case("hfoo","hfoo" => true)]
    #[test_case(1,1 => true)]
    #[test_case(1,2 => false)]
    fn works_eq<R: PartialEq + Clone + fmt::Display + fmt::Debug>(v1: R, v2: R) -> bool {
        let id1 = Id::<IdTag, R>::new(v1);
        let id2 = Id::<IdTag, R>::new(v2);
        id1 == id2
    }

    #[test_case("hoge" => true)]
    #[test_case("foo" => true)]
    #[test_case(1 => true)]
    #[test_case(2 => true)]
    fn works_clone<R: PartialEq + Clone + fmt::Display + fmt::Debug>(v1: R) -> bool {
        let id = Id::<IdTag, R>::new(v1.clone());
        id.clone().0 == v1
    }

    #[test_case("hoge" => "hoge")]
    #[test_case("foo" => "foo")]
    #[test_case(1 => "1")]
    #[test_case(2 => "2")]
    fn works_format<R: PartialEq + Clone + fmt::Display + fmt::Debug>(v1: R) -> String {
        let id = Id::<IdTag, R>::new(v1.clone());
        format!("{}", id)
    }
    #[test_case("hoge" => true)]
    #[test_case("foo" => true)]
    #[test_case(1 => true)]
    #[test_case(2 => true)]
    fn works_raw_id<R: PartialEq + Clone + fmt::Display + fmt::Debug>(v1: R) -> bool {
        let id = Id::<IdTag, R>::new(v1.clone());
        id.raw_id() == &v1
    }
}
