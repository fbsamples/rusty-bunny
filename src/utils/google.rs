/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_google_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let google_search_url = format!("https://google.com/search?q={}", encoded_query);

    google_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_google_search_url() {
        let fake_query = "hello";
        assert_eq!(
            construct_google_search_url(fake_query),
            "https://google.com/search?q=hello"
        );
    }
    #[test]
    fn test_construct_google_search_url_with_encoding() {
        let fake_query = "hello world";
        assert_eq!(
            construct_google_search_url(fake_query),
            "https://google.com/search?q=hello%20world"
        );
    }
}
