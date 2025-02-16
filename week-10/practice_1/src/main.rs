fn main() {
    let v = vec![101, 250, 330, 400];
    //vector v owns the object in the heap

    //only a single variable holds the heapmemory at a given time
    let v2 = v;
    //here two variables own heap value,
    //two pointers to the same content is not allowed in rust

    //Rust is very smart in terms of memory access, so it detects a race condition
    //as two variables point to the same heap

    println!("{:?}", v);

}
