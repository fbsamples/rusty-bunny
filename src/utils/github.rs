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

pub fn construct_github_url(query: &str) -> String {
    if query == "gh" {
        let github_dotcom = "https://github.com";

        github_dotcom.to_string()
    } else {
        // Assume the other match is "gh page"
        let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
        let github_url = format!("https://github.com/{}", encoded_query);

        github_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_github_profile_url_with_gh() {
        let fake_query = "gh";
        assert_eq!(construct_github_url(fake_query), "https://github.com");
    }

    #[test]
    fn test_construct_github_profile_url_with_repo_url() {
        let fake_query = "gh facebook";
        assert_eq!(
            construct_github_url(fake_query),
            "https://github.com/facebook"
        );
    }

    #[test]
    fn test_construct_github_search_url_with_repo_url() {
        let fake_query = "gh facebook/docusaurus";
        assert_eq!(
            construct_github_url(fake_query),
            "https://github.com/facebook/docusaurus"
        );
    }
}
