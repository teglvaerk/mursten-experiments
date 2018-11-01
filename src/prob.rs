pub trait RandomVariable {
    fn sample(&self, u: fn() -> f32) -> f32;
}

pub struct Unif {
    a: f32,
    b: f32,
}

impl Unif {
    pub fn new(a: f32, b: f32) -> Unif {
        Unif { a, b }
    }
}

impl RandomVariable for Unif {
    fn sample(&self, u: fn() -> f32) -> f32 {
        let &Unif { a, b } = self;
        a + u() * (b - a)
    }
}

pub struct Bern {
    p: f32,
}

impl Bern {
    pub fn new(p: f32) -> Bern {
        Bern { p }
    }
    pub fn sucess(&self, u: fn() -> f32) -> bool {
        self.sample(u) >= 1.0
    }
}

impl RandomVariable for Bern {
    fn sample(&self, u: fn() -> f32) -> f32 {
        let &Bern { p } = self;
        if u() < p {
            1.0
        } else {
            0.0
        }
    }
}

pub struct Bin {
    n: usize,
    p: f32,
}

impl Bin {
    pub fn new(n: usize, p: f32) -> Bin {
        Bin { n, p }
    }
}

impl RandomVariable for Bin {
    fn sample(&self, u: fn() -> f32) -> f32 {
        let &Bin { n, p } = self;
        (0..n).map(|_| Bern::new(p)).filter(|b| b.sucess(u)).count() as f32
        // TODO: Calculate the probability of each possible value (from 0 to n)
        // todo: and then use a weighted choice between them (using only one call to `u`)
    }
}

pub struct Exp {
    l: f32,
}

impl Exp {
    pub fn new(l: f32) -> Exp {
        Exp { l }
    }
}

impl RandomVariable for Exp {
    fn sample(&self, u: fn() -> f32) -> f32 {
        let &Exp { l } = self;
        (-1.0 / l) * (1.0 - u()).ln()
    }
}
