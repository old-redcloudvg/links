//! Scrape URLs from HTML and JavaScript easily.
//!
//! *HTML only scrapes URLs from <script>*
//! *JavaScript is not yet implemented*

/// Document type, either [`DocumentKind::HTML`] or [`DocumentKind::JavaScript`]
enum DocumentKind {
    /// An HTML document
    HTML,

    /// A JavaScript script
    JavaScript,
}

/// Describes a document
///
/// # See Also
///
/// [`Document::html`]
///
/// [`Document::js`]
pub struct Document<'a> {
    /// The kind of document
    kind: DocumentKind,

    /// The full text of the document
    text: &'a str,
}

impl<'a> Document<'a> {
    /// Wraps the given HTML code into a [`Document`].
    ///
    /// No syntax check is made for now.
    /// The given code is expected to be valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// links::Document::html("<html><head><title>Hello World</title></head><body><h1>Welcome</h1></html>");
    /// ```
    pub fn html(html: &'a str) -> Self {
        Document { kind: DocumentKind::HTML, text: html }
    }

    /// Wraps the given JavaScript script into a [`Document`]
    ///
    /// No syntax check is made for now.
    /// The given code is expected to be valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// links::Document::js("alert(1)");
    /// ```
    pub fn js(script: &'a str) -> Self {
        Document { kind: DocumentKind::JavaScript, text: script }
    }

    /// Returns string slices that contains absolute or relative URLs
    ///
    /// # Panics
    ///
    /// The method panics if the syntax of the document is invalid
    ///
    /// # Example
    ///
    /// ```rust
    /// assert_eq!(vec!["/main.js"], links::Document::html(r#"<script src="/main.js"></script>"#).urls());
    /// ```
    pub fn urls(&self) -> Vec<&'a str> {
        match self.kind {
            DocumentKind::HTML => todo!("get urls from html is not yet implemented"),
            DocumentKind::JavaScript => todo!("get urls from javascript is not yet implemented")
        }
    }
}
