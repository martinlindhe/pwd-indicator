use std::env;

fn main() {
    let cwd = match env::current_dir() {
        Ok(cwd) => cwd,
        Err(e) => panic!("error {:?}", e),
    };
    //println!("cwd {}", cwd.to_str().unwrap());

    // shows dir name of current working dir only
    let last = cwd.iter().last().unwrap().to_str().unwrap();
    print!("{}", last);
}
