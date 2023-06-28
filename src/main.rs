use std::env;
use std::fs;

// https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html

// fn listdir(path: &str) -> Option<fs::ReadDir> {
// fn listdir(path: &str) -> Option<Vec<fs::DirEntry>> {
fn listdir(path: &str) -> Vec<fs::DirEntry> {
    // python os.listdir
    // https://stackoverflow.com/a/26084812

    fs::read_dir(path)
        // .unwrap() // ignore error; should never be used at "top level"
        .expect("Not a dir: {path}") // raise
        .map(
            // closure -- equivalent to lambda x : x.method()
            // https://doc.rust-lang.org/rust-by-example/fn/closures.html
            |path| path.unwrap(),
        )
        .collect()

    // let rd = fs::read_dir(path);
    // Some(rd.unwrap())

    //
}

fn match_name(paths: Vec<fs::DirEntry>, name: &str, depth: usize) -> Option<fs::DirEntry> {
    for path in paths {
        let fname = path.file_name();
        if depth > 0 && fname.to_str().unwrap().starts_with(name) {
            println!("Partial match: {}", path.path().display());
            return Some(path);
        } else if fname == name {
            println!("Match: {}", path.path().display());
            return Some(path);
        }
    }
    // println!("No match: {name}");
    None
}

//fn sort_paths(mut paths: Vec<fs::DirEntry>) -> Vec<fs::DirEntry> {
//    // only collections can be sorted
//    // https://stackoverflow.com/a/40022060
//    //
//    // DirEntrys are not sortable, but their PathBufs are
//    paths.sort_by_key(|dir| dir.path());
//    //
//    // // alternatively, collect the PathBufs before sorting
//    // // all other attributes will be discarded
//    // let mut paths: Vec<_> = paths.map(|r| r.unwrap().path()).collect();
//    // paths.sort();
//    //
//    paths
//}

// fn get_dirs_only(paths: Vec<fs::DirEntry>) -> Vec<fs::DirEntry> {
//     // https://stackoverflow.com/a/44662455
//     paths
//         .into_iter()
//         .filter(|path| path.metadata().unwrap().is_dir())
//         // .cloned()
//         .collect()
// }

/// Simple program to descend into a root directory.
///
/// Given a root directory, as well as one or more successive subdirectory(s) to descend, the
/// directory tree is walked down until a subdirectory is not found.
///
/// # Examples
///
/// ```
/// $ checkdirs ~ '.config' 'mpv'
/// ```
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!(
            "Usage: {} <root> <dir1> [dir2] ...",
            env::current_exe() // yeah...
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        );
        std::process::exit(1);
    }

    // https://doc.rust-lang.org/rust-by-example/std_misc/path.html
    // let paths = listdir("/home");
    // let paths = sort_paths(paths);
    // let paths = get_dirs_only(paths);

    // let mut root = String::from(env::home_dir().unwrap().to_str().unwrap());
    // let names = [".cache", "xournal"];

    let mut root = String::from(&args[1]);
    let names = &args[2..];

    for (i, name) in names.iter().enumerate() {
        // println!("{}", root.display());
        let paths = listdir(&root);

        // https://www.sheshbabu.com/posts/rust-error-handling/
        let mat = match match_name(paths, name, i) {
            Some(res) => res.path(),
            None => std::process::exit(1),
        };

        // let mut root = root.join(m.file_name().unwrap().to_str().unwrap());

        // println!("{}", m.display());
        // let root = m.to_str().unwrap();

        // push_str is the only way i know that guarantees the variable is reused
        if root.chars().last().unwrap() != '/' {
            root.push_str("/");
        }
        root.push_str(mat.file_name().unwrap().to_str().unwrap());
    }
}
