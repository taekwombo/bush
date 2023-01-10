// https://www.youtube.com/watch?v=iVYWDIW71jk
// https://doc.rust-lang.org/reference/subtyping.html

pub fn strtok<'original>(string: &mut &'original str, delimiter: char) -> &'original str {
    if let Some(i) = string.find(delimiter) {
        let prefix = &string[0..i];
        let suffix = &string[(i + delimiter.len_utf8())..];

        *string = suffix;
        return prefix;
    }

    let prefix = &string[..];
    *string = "";

    prefix
}

pub fn strtok_single_lifetime<'a>(_string: &'a mut &'a str) -> &'a str { "" }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifetime() {
        let mut s = "one two";
        let prefix = strtok(&mut s, ' ');

        assert_eq!(prefix, "one");
        assert_eq!(s, "two");
    }

    #[allow(dead_code)]
    fn borrow_too_long() {
        let stat: &'static str = "static_string";
        let mut shorter = stat;
        let _shorter_prefix = strtok_single_lifetime(&mut shorter);
        // Above works fine since 'static is shorented the the lifetime of 'borrow_too_long.
    }
    
    #[cfg(feature = "failures")]
    #[test]
    fn static_borrow() {
        //! Does not compile because function strtok_single_lifetime exclusively borrows
        //! its argument for the lifetime of the argument (forever in the eyes of the argument).
        //! So, an exclusive reference to static string with anonymous lifetime becomes
        //! an exclusive reference with static lifetime to static string.
        //! ```
        //! fn(&'a mut &'a str) -> fn(&'static mut &'static str)
        //! ```
        let mut stat: &'static str = "static_string";
        let _prefix = strtok_single_lifetime(&mut stat);
    }
}
