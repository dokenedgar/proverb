pub fn build_proverb(list: &[&str]) -> String {
    let mut index = 0;
    let mut proverb: String = String::new();
    for item in list {
        if index == list.len() - 1 {
            if list.len() == 1 {
                proverb.push_str(format!("And all for the want of a {}.", list[0]).as_str());
            } else {
                proverb.push_str(format!("\nAnd all for the want of a {}.", list[0]).as_str());
            }
        } else {
            proverb = add_str(proverb, &item, list[index + 1], index);
            index += 1;
        }
    }

    proverb
}

fn add_str(mut initial_string: String, item: &str, next: &str, index: usize) -> String {
    if index > 0 {
        initial_string
            .push_str(format!("\nFor want of a {} the {} was lost.", item, next).as_str());
    } else {
        initial_string.push_str(format!("For want of a {} the {} was lost.", item, next).as_str());
    }
    initial_string
}
