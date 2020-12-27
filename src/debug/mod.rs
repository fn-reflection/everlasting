pub fn print_typeof<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}
