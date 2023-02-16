use fhq_treap::{TreapMap, TreapSet};

fn main() {
    let mut v = TreapMap::<u32, ()>::new();
    for i in 0..100 {
        v.insert(i, ());
    }
    for (key, value) in v.rev_slice(0..v.len()) {
        println!("{key}");
    }
}
