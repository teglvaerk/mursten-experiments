pub trait Data {}

pub trait Processor<D>
where
    D: Data,
{
    fn process(&mut self, D) -> D;
}

pub trait Chain<D>
where
    D: Data,
    Self: Sized,
{
    fn process(&mut self, D) -> D;
    fn add<P: Processor<D> + Sized>(self, processor: P) -> Node<D, P, Self> {
        Node {
            next: Some(self),
            processor,
            phantom_data: None,
        }
    }
}

pub struct Node<D, P, C>
where
    D: Data,
    P: Processor<D>,
    C: Chain<D>,
{
    next: Option<C>,
    processor: P,
    phantom_data: Option<Box<D>>,
}

impl<D, P, C> Chain<D> for Node<D, P, C>
where
    D: Data,
    P: Processor<D>,
    C: Chain<D>,
{
    fn process(&mut self, data: D) -> D {
        let data = match self.next {
            Some(ref mut node) => node.process(data),
            None => data,
        };
        self.processor.process(data)
    }
}

pub struct System;

impl<D> Chain<D> for System
where
    D: Data,
{
    fn process(&mut self, data: D) -> D {
        data
    }
}

////////////////////////////////

#[cfg(test)]
mod test {
    use drafts::recursive_template::{Chain, Data, Processor, System};

    impl Data for String {}

    struct PhysicsProcessor {}

    impl Processor<String> for PhysicsProcessor {
        fn process(&mut self, data: String) -> String {
            data + " => physics!"
        }
    }

    struct RenderProcessor {
        screen: String,
    }

    impl Processor<String> for RenderProcessor {
        fn process(&mut self, data: String) -> String {
            data + " => rendering in screen " + &self.screen + "!"
        }
    }

    struct AiProcessor {}

    impl Processor<String> for AiProcessor {
        fn process(&mut self, data: String) -> String {
            data + " => thinking!"
        }
    }

    #[test]
    fn create_chain() {
        let mut c = System
            .add(PhysicsProcessor {})
            .add(AiProcessor {})
            .add(RenderProcessor { screen: "1".into() })
            .add(RenderProcessor { screen: "2".into() });
        assert_eq!(
            c.process("chobis".into()),
            "chobis => physics! => thinking! => rendering in screen 1! => rendering in screen 2!"
        );
    }
}
