use num_traits::Num;

pub trait VecSize<Scalar: Num> {
    fn mag_sqr(&self) -> Scalar;
    fn manhattan(&self) -> Scalar;
}
