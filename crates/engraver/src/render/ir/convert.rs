pub trait Convert<T, U> {
    fn convert_x(&self, x: T) -> U;
    fn convert_y(&self, y: T) -> U;
    fn convert_thickness(&self, thickness: T) -> U;
}
