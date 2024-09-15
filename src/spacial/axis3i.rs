pub enum Axis3i {
    /// [`Vector3i::y`] and [`Vector3i::z`] change while [`Vector3i::x`] does not.
    AroundX,
    /// [`Vector3i::x`] and [`Vector3i::z`] change while [`Vector3i::y`] does not.
    AroundY,
    /// [`Vector3i::x`] and [`Vector3i::y`] change while [`Vector3i::z`] does not.
    AroundZ,
}
