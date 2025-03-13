fn main() {
    let a = vec![1,2,3];
    let mut a_iter = a.iter();

    assert_eq!(a_iter.next(), Some(&2));
    assert_eq!(a_iter.next(), Some(&2));
    assert_eq!(a_iter.next(), Some(&3));
    assert_eq!(a_iter.next(), None);
    // let b = &a;

    // println!("{:?}", a);
    // println!("{:?}", &a);
    // println!("{:?}", b);
    // println!("{:?}", &b);

    // let mut s = String::from("Hello");

    // let s1 = &mut s;
    // let s2 = s;

    // s1.push_str(" World");

    // // println!("{:?}", s);
    // println!("{:?}", s1);

}
