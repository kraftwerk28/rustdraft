use std::fmt::Display;

#[allow(dead_code)]
pub fn print_arr<T>(arr: &[T]) where T: Display {
    let mut res = String::from("[");
    let len = arr.len();
    for (i, el) in arr.iter().enumerate() {
        res.push_str(&format!("{}", el));
        if i < len - 1 {
            res.push_str(", ");
        }
    }
    res.push(']');
    println!("{}", res);
}
