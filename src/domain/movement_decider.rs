pub trait MovementDecider {
    fn decide(&self) -> bool;
}
