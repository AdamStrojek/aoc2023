
pub struct DirtyScanner<'a> {
    pub source: &'a str,

}

impl<'a> DirtyScanner<'a> {
    pub fn new(source: &'a str) -> Self {
        DirtyScanner{ source }
    }

    pub fn next_pattern(&mut self, pat: fn(&char) -> bool) -> Option<&'a str> {
        let mut it = self.source.char_indices().skip_while(|(_, x)| !pat(x));
        let (start, _) = it.next().unwrap_or((self.source.len(), ' '));
        let mut it = it.skip_while(|(_, x)| pat(x));
        let (last, _) = it.next().unwrap_or((self.source.len(), ' '));

        let result: &'a str = &self.source[start..last];
        self.source = &self.source[last..];
        if !result.is_empty() { Some(result) } else {None}
    }

    pub fn next_number<T: std::str::FromStr>(&mut self) -> Option<T> {
        let result: &str = self.next_pattern(|x| x.is_ascii_digit())?;

        result.parse::<T>().ok()
    }

    pub fn next_word(&mut self) -> Option<&'a str> {
         self.next_pattern(char::is_ascii_alphabetic)
    }

    pub fn next_symbol(&mut self) -> Option<&'a str> {
        self.next_pattern(char::is_ascii_punctuation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_scanner() {
        let input = "TEST";
        let _scanner = DirtyScanner::new(&input);
    }

    #[test]
    fn scanner_next_pattern() {
        let input = "111AAA";
        let mut scanner = DirtyScanner::new(&input);
        let result = scanner.next_pattern(char::is_ascii_digit);
        assert_eq!(&input[0..3], result.unwrap());
    }

    #[test]
    fn scanner_next_number() {
        let mut scanner = DirtyScanner::new("6969");
        assert_eq!(scanner.next_number::<u32>().unwrap(), 6969);
        let mut scanner = DirtyScanner::new("420AAA");
        assert_eq!(scanner.next_number::<u32>().unwrap(), 420);
        let mut scanner = DirtyScanner::new("111AAA");
        assert_eq!(scanner.next_number::<u32>().unwrap(), 111);
        let mut scanner = DirtyScanner::new("BBB333AAA");
        assert_eq!(scanner.next_number::<u32>().unwrap(), 333);
        let mut scanner = DirtyScanner::new("CCC123DDD987");
        assert_eq!(scanner.next_number::<u32>().unwrap(), 123);
        assert_eq!(scanner.next_number::<u32>().unwrap(), 987);
        let mut scanner = DirtyScanner::new("AAA");
        assert_eq!(scanner.next_number::<u32>(), None);
    }

    #[test]
    fn scanner_next_word() {
        assert!(char::is_ascii_alphabetic(&'a'));
        assert!(!char::is_ascii_alphabetic(&'0'));
        assert!(!char::is_ascii_alphabetic(&'9'));

        let mut scanner = DirtyScanner::new("6969");
        assert_eq!(scanner.next_word(), None);
        let mut scanner = DirtyScanner::new("AAA");
        assert_eq!(scanner.next_word().unwrap(), "AAA");
        let mut scanner = DirtyScanner::new("420BBB");
        assert_eq!(scanner.next_word().unwrap(), "BBB");
        let mut scanner = DirtyScanner::new("CCC111");
        assert_eq!(scanner.next_word().unwrap(), "CCC");
        let mut scanner = DirtyScanner::new("44DDDDD55");
        assert_eq!(scanner.next_word().unwrap(), "DDDDD");
        let mut scanner = DirtyScanner::new("BBB333AAA");
        assert_eq!(scanner.next_word().unwrap(), "BBB");
        assert_eq!(scanner.next_word().unwrap(), "AAA");
        let mut scanner = DirtyScanner::new("CCC123DDD987");
        assert_eq!(scanner.next_word().unwrap(), "CCC");
        assert_eq!(scanner.next_word().unwrap(), "DDD");
    }

}