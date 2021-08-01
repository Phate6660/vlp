fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let dir = if args.get(1) == None || args[1] == "." {
        std::env::var("PWD").unwrap()
    } else {
        args.get(1).unwrap().to_string()
    };
    let dir_vec = dir.split('/').collect::<Vec<&str>>();
    let dir_vec_count = dir_vec.iter().count();
    let mut n=0;
    for segment in dir_vec {
        if n == 0 {
            n = n + 1;
            continue;
        }
        if n < dir_vec_count - 1 {
            let char = segment.chars().collect::<Vec<char>>()[0];
            print!("/{}", char);
        } else if n == dir_vec_count - 1 {
            println!("/{}", segment);
        }
        n = n + 1;
    }
}
