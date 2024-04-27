use clap::Parser;
use clio::Output;
use std::fs;
use url::Url;

#[derive(Parser)]
struct Args {
    /// URL to use
    #[arg(required = true, short, long)]
    url: Url,
    /// Start of the enumeration
    #[arg(short, long, default_value_t = 1)]
    start: usize,
    /// End of the enumeration
    #[arg(required = true, short, long)]
    end: usize,
    /// Output file. If not provided, the output will be printed to stdout.
    /// If the file already exists, it will be overwritten
    #[arg(short, long, default_value = "-")]
    output: Output,
}

fn main() {
    // Get all the inputs
    let args = Args::parse();

    if args.start > args.end {
        println!("Start must be less than end");
        return;
    }

    // Identify the position of the incremental number in the URL path
    let (start_index, end_index, padding) = find_enumeration_position(&args.url.path()).unwrap();

    // Append to a buffer new lines with the incremental number replaced
    let mut buffer = String::with_capacity((args.end - args.start + 1) * args.url.as_str().len());
    for i in args.start..=args.end {
        let new_url = generate_new_url(&args.url, i, start_index, end_index, padding);
        buffer.push_str(&(format!("{}\n", new_url)));
    }

    // Write the buffer to the output
    if args.output.is_std() {
        println!("{}", buffer);
    } else if args.output.is_local() {
        fs::write(args.output.path().path(), &buffer).expect("Failed to write to file");
    } else {
        panic!("Unsupported output");
    }
}

/// Finds the position of the incremental number in the URL path
///
/// # Arguments
///
/// * `path` - The URL path
///
/// # Returns
///
/// A tuple containing the start index, end index, and padding of the incremental number
fn find_enumeration_position(path: &str) -> Result<(usize, usize, usize), String> {
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
    if start_index == 0 && end_index == 0 {
        Err("No incremental number found in URL path".to_string())
    } else {
        Ok((start_index, end_index, padding))
    }
}

/// Generates a new URL with the incremental number replaced
///
/// # Arguments
///
/// * `url` - The original URL
/// * `number` - The incremental number
/// * `start_index` - The start index of the sequence in the URL path
/// * `end_index` - The end index of the sequence in the URL path
/// * `padding` - The padding of the sequence
///
/// # Returns
///
/// The new URL with the incremental number replaced
fn generate_new_url(
    url: &Url,
    number: usize,
    start_index: usize,
    end_index: usize,
    padding: usize,
) -> Url {
    let new_path = &format!(
        "{}{}{}",
        &url.path()[..start_index],
        &format!("{:0width$}", number, width = padding),
        &url.path()[end_index + 1..]
    );
    Url::parse(&format!(
        "{}://{}{}{}",
        url.scheme(),
        url.host_str().unwrap(),
        new_path,
        url.query().map_or("", |q| q)
    ))
    .expect("Failed to parse new URL")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_enumeration_position() {
        let path = "/path/123";
        let result = find_enumeration_position(path).unwrap();
        assert_eq!(result, (6, 8, 3));

        let path = "/path/001";
        let result = find_enumeration_position(path).unwrap();
        assert_eq!(result, (6, 8, 3));

        let path = "/path/01";
        let result = find_enumeration_position(path).unwrap();
        assert_eq!(result, (6, 7, 2));

        let path = "/path/1";
        let result = find_enumeration_position(path);
        assert!(result.is_err());

        let path = "/path/001.foo1";
        let result = find_enumeration_position(path).unwrap();
        assert_eq!(result, (6, 8, 3));

        let path = "/path/foo1/001";
        let result = find_enumeration_position(path).unwrap();
        assert_eq!(result, (11, 13, 3));

        let path = "/path/abc";
        let result = find_enumeration_position(path);
        assert!(result.is_err());
    }
}
