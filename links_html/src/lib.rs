use xmlparser::Token;

pub fn get_urls_from_html(html: &str) -> Vec<&str> {
    let mut attr = None;
    let mut urls = vec![];

    for token in xmlparser::Tokenizer::from(html) {
        println!("{:?}", token);
        if let Ok(token) = token {
            if let Token::ElementStart { local, .. } = token {
                if local == "script" {
                    attr = Some("src")
                }
            } else if let Token::Attribute { local, value, .. } = token {
                if Some(local.as_str()) == attr {
                    urls.push(value.as_str());
                }
            } else if let Token::ElementEnd { .. } = token {
                attr = None
            }
        }
    }

    urls
}

#[cfg(test)]
mod tests {
    use crate::get_urls_from_html;

    #[test]
    fn simple() {
        assert_eq!(vec!["/main.js"], get_urls_from_html(r#"<script src="/main.js"></script>"#));
        assert_eq!(vec!["/main.js"], get_urls_from_html(r#"<script src="/main.js" /"#));
    }
}
