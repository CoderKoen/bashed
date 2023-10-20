pub fn tokenize(text: &str) -> (String, Vec<String>) {
    let mut main_command = String::new();
    let mut args: Vec<String> = Vec::new();
    let mut space_count = 0;
    let mut spliced_string = String::new();

    for c in text.chars() {
        spliced_string.push(c);

        if c == ' ' && space_count == 0 {
            main_command = spliced_string.clone();
            space_count += 1;
            spliced_string.clear();
        } else if c == ' ' && space_count > 0 {
            args.push(spliced_string.clone());
            space_count += 1;
            spliced_string.clear();
        }
    }

    // Check if there is a word after the last space
    if space_count > 0 && !spliced_string.is_empty() {
        args.push(spliced_string);
    } else if space_count == 0 {
        main_command = spliced_string;
    }

    (main_command, args)
}
