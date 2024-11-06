pub fn run_unsafe() {
    let address = 0x01234usize;
    let r = address as *mut i32;
    let mut n = 2;

    let naddress = &mut n as *mut i32;

    let values: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(naddress, 3) };
    values[0] = 12i32;
    values[1] = 11i32;

    println!("slice {:?}", values);
    println!("naddress {:?}", naddress);
}
