pub fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    let full2 = &full;
    full.push_str(" Esq");
    full
}

pub fn ascii_capitalize(v: &mut Vec<String>) {
    let c = &v[0];
    println!("first is {}", c);
}
