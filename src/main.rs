use std::io;
use url::Url;

fn main() {
    // Get all the inputs
    println!("Enter the URL: ");
    let mut url = String::new();

    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");

    let parsed_url = Url::parse(&url).expect("Failed to parse URL");

    println!("Enter sequence start: ");
    let mut start = String::new();

    io::stdin()
        .read_line(&mut start)
        .expect("Failed to read line");

    let start: usize = start.trim().parse().expect("Failed to parse start");

    println!("Enter sequence end: ");
    let mut end = String::new();

    io::stdin()
        .read_line(&mut end)
        .expect("Failed to read line");

    let end: usize = end.trim().parse().expect("Failed to parse end");

    if start > end {
        println!("Start must be less than end");
        return;
    }

    // Identify the position of the sequence in the URL path
    let path = parsed_url.path();
    let mut start_index = 0;
    let mut end_index = 0;
    let mut padding = 1;
    for i in (0..path.len()).rev() {
        if path.chars().nth(i).unwrap().is_numeric() {
            let mut j = i;
            while j > 0 && path.chars().nth(j - 1).unwrap().is_numeric() {
                j -= 1;
            }
            if j == i {
                continue;
            }
            padding = i - j + 1;
            start_index = j;
            end_index = i;
            break;
        }
    }

    // Print new lines with the sequence replaced
    for i in start..=end {
        let new_path = &format!(
            "{}{}{}",
            &path[..start_index],
            &format!("{:0width$}", i, width = padding),
            &path[end_index + 1..]
        );
        let new_url = Url::parse(&format!(
            "{}://{}{}{}",
            parsed_url.scheme(),
            parsed_url.host_str().unwrap(),
            new_path,
            parsed_url.query().map_or("", |q| q)
        ))
        .expect("Failed to parse new URL");
        println!("{}", new_url);
    }
}
