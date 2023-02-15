use fhq_treap::TreapMap;

fn main() {
    let mut v: TreapMap<i32, i32> = [(2, 3), (5, 4), (4, 3)].into_iter().collect();
    println!("{:?}", v.remove(&5));
}
