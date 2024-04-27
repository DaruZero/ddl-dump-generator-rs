use clap::Parser;
use clio::*;
use std::fs;
use url::Url;

#[derive(Parser)]
struct Args {
    /// URL to use
    #[arg(required = true, short, long)]
    url: Url,
    /// Start of the sequence
    #[arg(short, long, default_value_t = 1)]
    start: usize,
    /// End of the sequence
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

    // Identify the position of the sequence in the URL path
    let path = args.url.path();
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

    // Append to a buffer new lines with the sequence replaced
    let mut buffer = String::with_capacity((args.end - args.start + 1) * args.url.as_str().len());
    for i in args.start..=args.end {
        let new_path = &format!(
            "{}{}{}",
            &path[..start_index],
            &format!("{:0width$}", i, width = padding),
            &path[end_index + 1..]
        );
        let new_url = Url::parse(&format!(
            "{}://{}{}{}",
            args.url.scheme(),
            args.url.host_str().unwrap(),
            new_path,
            args.url.query().map_or("", |q| q)
        ))
        .expect("Failed to parse new URL");

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
