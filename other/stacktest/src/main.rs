mod stack;

fn main() {
    println!("Hello, world!");
}

#[test]
fn initiate_stack() {
    let _x: stack::Stack<u32> = stack::Stack::new(None);
}

#[test]
fn push_test() {
    let mut x: stack::Stack<u32> = stack::Stack::new(None);
    x.push(3);
    x.push(4);
    assert_eq!(x.elements, vec![3, 4])
}

#[test]
fn pop_test() {
    let mut x: stack::Stack<u32> = stack::Stack::new(None);
    x.push(3);
    x.push(4);
    x.pop();
    assert_eq!(x.elements, vec![3])
}

#[test]
fn overpopping() {
    let mut x: stack::Stack<u32> = stack::Stack::new(None);
    x.push(3);
    x.pop();
    x.pop();
}

#[test]
fn overpushing() {
    let mut x: stack::Stack<u32> = stack::Stack::new(Some(4));

    for i in 0..5 {
        x.push(i)
    }

}

#[test]
fn full_test() {
    let mut x: stack::Stack<u32> = stack::Stack::new(Some(4));

    for i in 0..4 {
        x.push(i)
    }

    assert_eq!(x.elements, vec![0, 1, 2, 3]);
    assert_eq!(x.is_full(), true);


    let mut y: stack::Stack<u32> = stack::Stack::new(Some(4));

    for i in 0..3 {
        y.push(i)
    }

    assert_eq!(y.elements, vec![0, 1, 2]);
    assert_eq!(y.is_full(), false);

    let mut y: stack::Stack<u32> = stack::Stack::new(None);

    for i in 0..5 {
        y.push(i)
    }

    assert_eq!(y.elements, vec![0, 1, 2, 3, 4]);
    assert_eq!(y.is_full(), false);
}

#[test]
fn empty_test() {
    let mut x: stack::Stack<u32> = stack::Stack::new(None);
    assert_eq!(x.is_empty(), true);

    x.push(5);
    x.push(3);

    assert_eq!(x.elements, vec![5, 3]);

    x.pop();
    x.pop();

    assert_eq!(x.is_empty(), true);
}

#[test]
fn pop_return() {
    let mut x: stack::Stack<u32> = stack::Stack::new(None);
    x.push(4);
    assert_eq!(x.pop(), Some(4));
    assert_eq!(x.pop(), None);
}
