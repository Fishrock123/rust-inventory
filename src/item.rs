
pub trait Item {
    fn item_type(&self) -> &str;

    fn max_stack_size(&self) -> i32;
}
