fn my_function(data: &(u32, &str)) {
    let (my_num, my_str) = data;
    println!("my_num: {}, my_str: {}", my_num, my_str);
}

fn main() {
    println!("Hello, world!");

    let data = (4, "OK");
    my_function(&data);
}
