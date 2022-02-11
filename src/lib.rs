/*
| MIT License
|
| Copyright (c) 2022 Mauro Baladés
|
| Permission is hereby granted, free of charge, to any person obtaining a copy
| of this software and associated documentation files (the "Software"), to deal
| in the Software without restriction, including without limitation the rights
| to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
| copies of the Software, and to permit persons to whom the Software is
| furnished to do so, subject to the following conditions:
|
| The above copyright notice and this permission notice shall be included in all
| copies or substantial portions of the Software.
|
| THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
| IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
| FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
| AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
| LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
| OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
| SOFTWARE.
*/

use regex::Regex;

const URL_REGEX: &str =
    r"[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)";

pub(crate) fn is_url(url: &str) -> bool {
    let re = Regex::new(URL_REGEX).unwrap();
    return re.is_match(url);
}

#[cfg(test)]
mod tests {
    use crate::is_url;

    #[test]
    fn normal_url() {
        assert!(is_url("https://hello.com"));
    }

    #[test]
    fn url_path() {
        assert!(is_url("https://hello.com/example"));
    }

    #[test]
    fn url_section() {
        assert!(is_url("https://hello.com#example"));
    }

    #[test]
    fn url_arguments() {
        assert!(is_url("https://hello.com?q=hello"));
    }
}
