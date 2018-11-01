trait Outcome {}

trait ProbabilitySpace<O>
where
    O: Outcome,
{
}

trait Event<O>
where
    O: Outcome,
{
    fn contains_outcome(&self, o: O) -> bool;
    fn intersection(&self, other: &Self) -> Self;
    fn union(&self, other: &Self) -> Self;
    fn lebesgue_measure(&self) -> f32;
}

trait DistributionFunction<O>
where
    O: Outcome,
{
}

trait RandomVariable<O, E>
where
    O: Outcome,
    E: Event<O>,
{
    fn probability_of(&self, ev: E) -> f32;
}

#[cfg(test)]
mod test {

    use drafts::prob2::*;

    #[test]
    fn desired_continuous_usage() {
        let va_x = Unif::new(0.0, 1.0);
        let ev_a = Range::new(0.0, 0.3);
        let p_win = va_x.probability_of(ev_a);

        assert_eq!(p_win, 0.3);
    }

    #[derive(Clone)]
    struct Unif {
        a: f32,
        b: f32,
    }

    impl Unif {
        fn new(a: f32, b: f32) -> Self {
            if a > b {
                panic!("Unif::new, a={} must be less or equal than b={}", a, b)
            }
            Unif { a, b }
        }
    }

    #[derive(Clone)]
    enum Range {
        Simple(f32, f32),
        Union(Box<Range>, Box<Range>),
        Empty,
    }

    impl Range {
        fn new(a: f32, b: f32) -> Range {
            if a > b {
                Range::Empty
            } else {
                Range::Simple(a, b)
            }
        }
        fn unmesaurable(&self) -> bool {
            if let Range::Simple(a, b) = *self {
                a == b
            } else {
                false
            }
        }
    }

    impl Outcome for f32 {}

    impl Event<f32> for Range {
        fn contains_outcome(&self, x: f32) -> bool {
            match *self {
                Range::Empty => false,
                Range::Simple(a, b) => a < x && x < b,
                Range::Union(ref s, ref t) => s.contains_outcome(x) || t.contains_outcome(x),
            }
        }
        fn intersection(&self, other: &Self) -> Self {
            match *self {
                Range::Empty => Range::Empty,
                Range::Union(ref s, ref t) => Range::Union(
                    Box::new(s.intersection(other)),
                    Box::new(s.intersection(other)),
                ),
                Range::Simple(a, b) => match *other {
                    Range::Empty => Range::Empty,
                    Range::Union(ref s, ref t) => Range::Union(
                        Box::new(s.intersection(self)),
                        Box::new(t.intersection(self)),
                    ),
                    Range::Simple(c, d) => {
                        if a > d || b < c {
                            Range::Empty
                        } else {
                            Range::Simple(a.max(c), b.min(d))
                        }
                    }
                },
            }
        }
        fn union(&self, other: &Self) -> Self {
            match *self {
                Range::Empty => (*other).clone(),
                Range::Union(ref s, ref t) => {
                    Range::Union(Box::new(s.union(other)), Box::new(s.union(other)))
                }
                Range::Simple(a, b) => match *other {
                    Range::Empty => (*self).clone(),
                    Range::Union(ref s, ref t) => {
                        Range::Union(Box::new(s.union(self)), Box::new(t.union(self)))
                    }
                    Range::Simple(c, d) => {
                        if a > d || b < c {
                            Range::Union(Box::new((*self).clone()), Box::new((*other).clone()))
                        } else {
                            Range::Simple(a.min(c), b.max(d))
                        }
                    }
                },
            }
        }
        fn lebesgue_measure(&self) -> f32 {
            match *self {
                Range::Empty => 0.0,
                Range::Union(ref s, ref t) => {
                    s.lebesgue_measure() + t.lebesgue_measure()
                        - s.intersection(&t).lebesgue_measure()
                }
                Range::Simple(a, b) => b - a,
            }
        }
    }

    impl RandomVariable<f32, Range> for Unif {
        fn probability_of(&self, ev: Range) -> f32 {
            let s = Range::Simple(self.a, self.b);
            s.intersection(&ev).lebesgue_measure() / s.lebesgue_measure()
        }
    }

    #[test]
    fn more_assertions() {
        let va_x = Unif::new(0.0, 80.0);
        assert_eq!(1.00, va_x.probability_of(Range::new(0.0, 80.0)));
        assert_eq!(0.25, va_x.probability_of(Range::new(-20.0, 20.0)));
        assert_eq!(0.25, va_x.probability_of(Range::new(0.0, 20.0)));
        assert_eq!(1.00, va_x.probability_of(Range::new(0.0, 100.0)));
    }

    #[ignore]
    #[test]
    fn transform_random_variables() {
        let va_x = Unif::new(0.0, 10.0);
        let va_y = va_x.add_constant(10.0);
        assert_eq!(
            va_x.probability_of(Range::new(2.0, 4.0)),
            va_y.probability_of(Range::new(12.0, 14.0))
        );
        assert_eq!(va_y.probability_of(Range::new(0.0, 8.0)), 0.0);
    }

    use std::ops::Add;

    trait AddConstant<E, O>
    where
        Self: RandomVariable<O, E> + Sized,
        E: Event<O>,
        O: Outcome + Add,
    {
        fn add_constant(&self, k: f32) -> AddedConstantVariable<Self, E, O>;
    }

    struct AddedConstantVariable<RV, E, O>
    where
        RV: RandomVariable<O, E>,
        E: Event<O>,
        O: Outcome + Add,
    {
        v: RV,
        k: f32,
        phanthom: Option<(E, O)>,
    }

    impl<RV, E, O> AddConstant<E, O> for RV
    where
        RV: RandomVariable<O, E> + Sized + Clone,
        E: Event<O>,
        O: Outcome + Add,
    {
        fn add_constant(&self, k: f32) -> AddedConstantVariable<RV, E, O> {
            AddedConstantVariable {
                v: (*self).clone(),
                k,
                phanthom: Option::None,
            }
        }
    }

    impl<RV, E, O> RandomVariable<O, E> for AddedConstantVariable<RV, E, O>
    where
        RV: RandomVariable<O, E>,
        E: Event<O>,
        O: Outcome + Add,
    {
        fn probability_of(&self, ev: E) -> f32 {
            self.v.probability_of(ev)
        }
    }
}
