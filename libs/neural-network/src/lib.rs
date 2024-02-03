pub struct Network {

}
impl Network {
    // pub fn propagate

    fn foo_3(items: &mut [f32]) {
        items.push(1.0);
        //   ^^^^^  ----------
        //  binding    type
        // (immut.)  (mutable)
    }
}