pub mod tilde_expansion;

#[cfg(test)]
mod tests {
    use crate::tilde_expansion::ExpandTilde;

    #[test]
    fn it_works() {
        assert_eq!(
            "~/Documents/oxygene lessons booking.pdf"
                .expand_tilde()
                .unwrap(),
            std::path::PathBuf::from("/home/jedwards/Documents/oxygene lessons booking.pdf")
        );
    }
}
