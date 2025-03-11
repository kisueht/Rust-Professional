pub fn new_count_distinct(input_str: &str) -> usize {
    let mut input_c:Vec<&str> = input_str.split(',').collect();
    input_c.sort_unstable();

    let mut index = 1;
    while index < input_c.len() {
        if input_c[index] == input_c[index - 1] {
            input_c.remove(index - 1);
        } else {
            index += 1;
        }
    }
    
    input_c.len()
}
