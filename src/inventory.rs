
use super::item::Item;


pub struct Inventory<T> {
    slots: Vec<Option<(T, i32)>>,
}

impl<T: Item + Clone> Inventory<T> {

    pub fn new(length: usize) -> Inventory<T> {
        let mut new_slots = Vec::<Option<(T, i32)>>::with_capacity(length);
        let mut i = length;
        while i > 0 {
            new_slots.push(None);
            i -= 1;
        }
        Inventory { slots: new_slots }
    }

    pub fn len(&self) -> usize {
        self.slots.len()
    }

    pub fn give(&mut self, item: T, amount: i32) -> bool {
        let item_type: &str = item.item_type();
        if !self.has_space(&item, amount) {
            return false;
        }
        let mut sum: i32 = amount;
        for slot in self.slots.iter_mut() {
            if let &mut Some((ref other, ref mut stack_size)) = slot {
                if item_type != other.item_type() { continue; }

                let remainder = other.max_stack_size() - *stack_size;
                if remainder >= sum {
                    *stack_size += sum;
                    return true;
                } else {
                    *stack_size += remainder;
                    sum -= remainder;
                }

                if sum < 0 {
                    unreachable!();
                }
            }
        }
        for slot in self.slots.iter_mut() {
            if let &mut None = slot {
                let stack_size: i32 = item.max_stack_size();
                if stack_size >= sum {
                    *slot = Some((item.clone(), sum));
                } else {
                    *slot = Some((item.clone(), stack_size));
                }

                sum -= stack_size;
                if sum <= amount { return true; }
            }
        }
        unreachable!();
    }

    pub fn take(&mut self, item_type: &str, amount: i32) -> bool {
        if self.get_amount(item_type) < amount {
            return false;
        }
        let mut sum = amount;
        for slot in self.slots.iter_mut() {
            let mut remove = false;
            if let &mut Some((ref other, ref mut stack_size)) = slot {
                if item_type != other.item_type() { continue; }

                let remainder = amount - sum;
                if *stack_size > remainder {
                    *stack_size -= remainder;
                    break;
                } else {
                    sum += *stack_size;
                    remove = true;
                }
            }
            if remove {
                *slot = None;
            }
        }
        true
    }

    // TODO: insertion and removal from specific slots
    //
    // pub fn insert(&self, item: T, amount: i32, location: usize) -> Slot {
    //     unimplemented!();
    // }
    //
    // pub fn withdraw(&self, location: usize) -> Slot {
    //     unimplemented!();
    // }

    fn has_space<'a>(&self, item: &'a T, amount: i32) -> bool {
        let item_type: &str = item.item_type();
        let mut sum: i32 = 0;
        for slot in self.slots.iter() {
            if let &Some((ref other, stack_size)) = slot {
                if item_type != other.item_type() { continue; }

                sum += other.max_stack_size() - stack_size;

                if sum >= amount { return true; }
            }
        }
        for slot in self.slots.iter() {
            if let &None = slot {
                sum += item.max_stack_size();

                if sum >= amount { return true; }
            }
        }
        false
    }

    fn get_amount(&self, item_type: &str) -> i32 {
        let mut sum: i32 = 0;
        for slot in self.slots.iter() {
            if let &Some((ref other, amount)) = slot {
                if item_type != other.item_type() { continue; }
                sum += amount;
            }
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::item::Item;

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
    fn empty_by_default() {

        impl<T> Inventory<T> {
            fn test_empty(&self) {
                for slot in self.slots.iter() {
                    if let &Some(_) = slot {
                        panic!("inv should be empty");
                    }
                }
            }
        }

        let inv = Inventory::<TestItem>::new(5);
        inv.test_empty();
    }
}
