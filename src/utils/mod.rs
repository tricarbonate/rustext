

/*
 * Force program exit
 */
pub fn die(err: Option<std::io::Error>) {
    print!("{}", termion::clear::All);
    match err {
        Some(e) => panic!("{}", e),
        None => panic!(),
    }
}
