pub struct NvsPartition<T>(pub(crate) T);
pub struct Nvs<T>(pub(crate) T);

// clone method required for both linux as well as espidf
impl<T> Clone for NvsPartition<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
