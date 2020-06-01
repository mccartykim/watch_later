fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => search_by_title(args[1]),
        3 => search_by_title_and_year(args[1], args[2]),
        _ => invalid_input()
    }
}

fn search_by_title(title: &str) {
    
}

fn search_by_title_and_year(title: &str, year: &str) {
    
}

fn invalid_input() {
    
}

fn print_help() {
    
}
