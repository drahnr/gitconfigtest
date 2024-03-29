use git2;

fn main() {
    println!("Hello, world!");

    let loco = std::env::current_dir().expect("CWD env var not set properly");
	let repo = git2::Repository::open(&loco).expect("cwd should be a repo");
    let cfg = repo.config().expect("Failed to obtain config struct");
    for entry in &cfg.entries(None).unwrap() {
        let entry = entry.unwrap();
        println!("{} => {}", entry.name().unwrap(), entry.value().unwrap());
    }
    let name = cfg.get_str("user.name").unwrap();
    let email = cfg.get_str("user.email").unwrap();
    println!("Found {} <{}> ", name, email);
}
