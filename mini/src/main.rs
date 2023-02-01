use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Zuhang Xu",
    about = "Output the permutation of the input string"
)]

struct Opt {
    #[clap(short, long)]
    input: String,
}

// main function that prints the permutation of the input string
fn main() {
    let opt = Opt::parse();
    let input = opt.input;
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort();
    let mut result: Vec<String> = Vec::new();
    permute(&mut chars, &mut result, 0, input.len() - 1);
    for i in result {
        println!("{}", i);
    }
}

// permute function that generates the permutation of the input string
fn permute(chars: &mut Vec<char>, result: &mut Vec<String>, start: usize, end: usize) {
    if start == end {
        result.push(chars.iter().collect());
    } else {
        for i in start..=end {
            chars.swap(start, i);
            permute(chars, result, start + 1, end);
            chars.swap(start, i);
        }
    }
}
