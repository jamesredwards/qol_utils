use std::path::{Path, PathBuf};

pub trait ExpandTilde {
    fn expand_tilde(&self) -> Option<PathBuf>;
}

// impl ExpandTilde for str {
//     fn expand_tilde(&self) -> Option<PathBuf> {
//         let mut path = PathBuf::new();
//
//         if self.starts_with("~") {
//             if let Ok(home_dir) = std::env::var("HOME") {
//                 let mut expanded_path_str = String::from(home_dir);
//                 expanded_path_str.push_str(&self[1..]);
//                 path.push(expanded_path_str);
//                 Some(path)
//             } else {
//                 panic!("No Home dir found")
//             }
//         } else {
//             path.push(self);
//             Some(path)
//         }
//     }
// }
impl<T: AsRef<str>> ExpandTilde for T {
    fn expand_tilde(&self) -> Option<PathBuf> {
        expand_tilde(self.as_ref())
    }
}
impl ExpandTilde for Path {
    fn expand_tilde(&self) -> Option<PathBuf> {
        if let Some(path_str) = self.to_str() {
            return path_str.expand_tilde();
        }
        None
    }
}

fn expand_tilde(path_str: &str) -> Option<PathBuf> {
    let mut path = PathBuf::new();

    if path_str.starts_with("~") {
        if let Ok(home_dir) = std::env::var("HOME") {
            let mut expanded_path_str = String::from(home_dir);
            expanded_path_str.push_str(&path_str[1..]);
            path.push(expanded_path_str);
            Some(path)
        } else {
            panic!("No Home dir found")
        }
    } else {
        path.push(path_str);
        Some(path)
    }
}
