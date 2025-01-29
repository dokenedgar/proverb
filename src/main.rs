fn main() {
    let list = [
        "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
    ];
    let res = proverb::build_proverb(&list);
    print!("{res}");
}
