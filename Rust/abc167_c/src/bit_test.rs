fn main() {
    for i in 0..10 {
        // println!("{:b}", i);
        let bin = format!("{:08b}", i);
        print_typename(bin);
    }
}

fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}
