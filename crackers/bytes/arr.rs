extern crate alloc;

pub type Arr<T> = alloc::vec::Vec<T>;

pub(crate) fn Arr_with_start<T>(start: T) -> Arr<T> {
    alloc::vec![start]
}
