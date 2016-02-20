extern crate rust_inventory;
use rust_inventory::inventory::Inventory;
use rust_inventory::item::Item;


#[derive(Clone)]
struct TestItem {
    _data: i32,
}

impl Item for TestItem {
    fn item_type(&self) -> &str {
        "Legendary Motor Lance"
    }
    fn max_stack_size(&self) -> i32 {
        3
    }
}


#[test]
fn new_inventory() {
    let inv = Inventory::<TestItem>::new(5);
    assert_eq!(inv.len(), 5);
}

#[test]
fn give_take() {
    let mut inv = Inventory::<TestItem>::new(5);
    assert!(inv.give(TestItem { _data: 0 }, 3));
    assert!(inv.take("Legendary Motor Lance", 2));
    assert!(inv.give(TestItem { _data: 0 }, 1));
    assert!(inv.take("Legendary Motor Lance", 1));
    assert!(inv.take("Legendary Motor Lance", 2));
}
