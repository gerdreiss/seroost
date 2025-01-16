#[derive(Debug)]
pub(crate) struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.content.len() == 0
    }

    pub(crate) fn trim_start(&mut self) {
        while !self.content.is_empty() && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    pub(crate) fn take(&mut self, n: usize) -> &'a [char] {
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token
    }

    pub(crate) fn take_while<P>(&mut self, predicate: P) -> &'a [char]
    where
        P: Fn(&char) -> bool,
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }

        self.take(n)
    }

    pub(crate) fn next_token(&mut self) -> Option<String> {
        fn to_upper(slice: &[char]) -> String {
            slice.iter().map(|c| c.to_ascii_uppercase()).collect()
        }

        self.trim_start();
        if self.is_empty() {
            None
        } else if self.content[0].is_numeric() {
            Some(self.take_while(|c| c.is_numeric()).iter().collect())
        } else if self.content[0].is_alphabetic() {
            Some(to_upper(self.take_while(|c| c.is_alphanumeric())))
        } else {
            Some(to_upper(self.take(1)))
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
