#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// See [`retain`](Retain::retain) for more information.
pub trait Retain {
  /// Input
  type Input;
  /// Output
  type Output;

  /// Retains only the elements specified by the `F` predicate.
  fn retain(&mut self, input: Self::Input) -> Self::Output;
}

/// ```rust
/// let mut opt = Some(1);
/// cl_traits::Retain::retain(&mut opt, |n| n % 2 == 0);
/// assert_eq!(opt, None);
/// ```
impl<T> Retain for Option<T> {
  type Input = fn(&T) -> bool;
  type Output = ();

  #[inline]
  fn retain(&mut self, input: Self::Input) {
    if let Some(elem) = self.as_mut() {
      if !input(elem) {
        *self = None;
      }
    }
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// cl_traits::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure, &[2]);
/// ```
#[cfg(feature = "alloc")]
impl<T> Retain for Vec<T> {
  type Input = fn(&T) -> bool;
  type Output = ();

  #[inline]
  fn retain(&mut self, input: Self::Input) {
    self.retain(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// cl_traits::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure[..], &[2]);
/// ```
#[cfg(feature = "with-arrayvec")]
impl<A> Retain for arrayvec::ArrayVec<A>
where
  A: arrayvec::Array,
{
  type Input = fn(&A::Item) -> bool;
  type Output = ();

  #[inline]
  fn retain(&mut self, input: Self::Input) {
    self.retain(|i| input(i))
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::small_vec();
/// cl_traits::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure[..], &[2]);
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> Retain for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Input = fn(&A::Item) -> bool;
  type Output = ();

  #[inline]
  fn retain(&mut self, input: Self::Input) {
    self.retain(|i| input(i))
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::static_vec();
/// cl_traits::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure[..], &[2]);
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Retain for staticvec::StaticVec<T, N> {
  type Input = fn(&T) -> bool;
  type Output = ();

  #[inline]
  fn retain(&mut self, input: Self::Input) {
    self.retain(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// cl_traits::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure[..], &[2]);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Retain for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = fn(&A::Item) -> bool;
  type Output = ();

  #[inline]
  fn retain(&mut self, input: Self::Input) {
    self.retain(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// cl_traits::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure[..], &[2]);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Retain for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = fn(&A::Item) -> bool;
  type Output = ();

  #[inline]
  fn retain(&mut self, input: Self::Input) {
    self.retain(input)
  }
}
