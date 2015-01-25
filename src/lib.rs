/// Implements struct hierarchy and serializer for the [HAR 1.2 spec][1].
///
/// [1]: http://www.softwareishard.com/blog/har-12-spec/
extern crate serialize;

use serialize::{Encodable, Encoder};

const HAR_VERSION: &'static str = "1.2";
const HAR_CREATOR_NAME: &'static str = "Rust-HAR";
const HAR_CREATOR_VERSION: &'static str = "0.0.4";

/// This object represents the root of the exported data.
///
/// This object MUST be present and its name MUST be "log".
pub struct Log {
    /// Version number of the format.
    version: String,

    /// Name and version info of the log creator application.
    creator: Creator,

    /// Name and version info of used browser.
    browser: Option<Browser>,

    /// List of all exported (tracked) pages.
    /// Leave out this field if the application does not support grouping by pages.
    pages: Option<Vec<Page>>,

    /// List of all exported (tracked) requests.
    entries: Vec<Entry>,

    /// A comment provided by the user or the application.
    comment: Option<String>
}

impl Log {
    pub fn new(browser: Option<Browser>, comment: Option<String>) -> Log {
        Log {
            version: HAR_VERSION.to_string(),
            creator: Creator {
                name: HAR_CREATOR_NAME.to_string(),
                version: HAR_CREATOR_VERSION.to_string(),
                comment: None
            },
            browser: browser,
            pages: None,
            entries: Vec::new(),
            comment: comment
        }
    }

    pub fn add_page(&mut self, page: Page) {
        match self.pages {
            Some(ref mut pages) => pages.push(page),
            None => self.pages = Some(vec![page])
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
}

impl <S: Encoder<E>, E> Encodable<S, E> for Log {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        encoder.emit_struct("Log", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            fields.push(("version", Box::new(self.version.to_string())));
            fields.push(("creator", Box::new(&self.creator)));
            match self.browser {
                Some(ref browser) => fields.push(("browser", Box::new(browser))),
                None => ()
            }
            match self.pages {
                Some(ref pages) => fields.push(("pages", Box::new(pages.as_slice()))),
                None => ()
            }
            fields.push(("entries", Box::new(self.entries.as_slice())));
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}

/// This object contains information about the log creator application.
pub struct Creator {
    name: String,
    version: String,
    comment: Option<String>
}

impl <S: Encoder<E>, E> Encodable<S, E> for Creator {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        encoder.emit_struct("Creator", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            fields.push(("name", Box::new(self.name.to_string())));
            fields.push(("version", Box::new(self.version.to_string())));
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}


/// This object contains information about the browser that created the log.
pub struct Browser {
    name: String,
    version: String,
    comment: Option<String>
}

impl Browser {
    pub fn new(name: String, version: String, comment: Option<String>) -> Browser{
        Browser {
            name: name,
            version: version,
            comment: comment
        }
    }
}

impl <S: Encoder<E>, E> Encodable<S, E> for Browser {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        encoder.emit_struct("Browser", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            fields.push(("name", Box::new(self.name.to_string())));
            fields.push(("version", Box::new(self.version.to_string())));
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}


/// This object represents list of exported pages.
pub struct Page {
    /// Date and time stamp for the beginning of the page load
    /// (ISO 8601 YYYY-MM-DDThh:mm:ss.sTZD, e.g. 2009-07-24T19:20:30.45+01:00).
    started_date_time: String,
    /// Unique identifier of a page within the . Entries use it to refer the parent page.
    id: String,
    /// Page title.
    title: String,
    /// Detailed timing info about page load.
    page_timings: PageTimings,
    /// A comment provided by the user or the application.
    comment: Option<String>
}

impl Page {
    pub fn new(started_date_time: String,
               id: String,
               title: String,
               page_timings: PageTimings,
               comment: Option<String>) -> Page {
        Page {
            started_date_time: started_date_time,
            id: id,
            title: title,
            page_timings: page_timings,
            comment: comment
        }
    }
}

impl <S: Encoder<E>, E> Encodable<S, E> for Page {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        encoder.emit_struct("Page", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            fields.push(("startedDateTime", Box::new(self.started_date_time.to_string())));
            fields.push(("id", Box::new(self.id.to_string())));
            fields.push(("title", Box::new(self.title.to_string())));
            fields.push(("pageTimings", Box::new(&self.page_timings)));
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}


/// This object describes timings for various events (states) fired during the page load.
///
/// All times are specified in milliseconds.
/// If a time info is not available appropriate field is set to -1.
pub struct PageTimings {
    /// Content of the page loaded.
    /// Number of milliseconds since page load started (page.startedDateTime).
    /// Use -1 if the timing does not apply to the current request.
    on_content_load: OptionalTiming,

    /// Page is loaded (onLoad event fired).
    /// Number of milliseconds since page load started (page.startedDateTime).
    /// Use -1 if the timing does not apply to the current request.
    on_load: OptionalTiming,

    /// A comment provided by the user or the application.
    comment: Option<String>
}

impl PageTimings {
    pub fn new(
        on_content_load: OptionalTiming,
        on_load: OptionalTiming,
        comment: Option<String>
    ) -> PageTimings {
        PageTimings {
            on_content_load: on_content_load,
            on_load: on_load,
            comment: comment,
        }
    }
}

impl <S: Encoder<E>, E> Encodable<S, E> for PageTimings {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        encoder.emit_struct("PageTimings", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            fields.push(("onContentLoad", Box::new(self.on_content_load)));
            fields.push(("onLoad", Box::new(self.on_load)));
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}

/// This object represents an array with all exported HTTP requests. Sorting entries by
/// startedDateTime (starting from the oldest) is preferred way how to export data since it can
/// make importing faster. However the reader application should always make sure the array is
/// sorted (if required for the import).
pub struct Entry {
    /// Reference to the parent page (unique).
    /// Leave out this field if the application does not support grouping by pages.
    pageref: Option<String>,

    /// Date and time stamp of the request start (ISO 8601 YYYY-MM-DDThh:mm:ss.sTZD).
    started_date_time: String,

    /// Total elapsed time of the request in milliseconds.
    /// This is the sum of all timings available in the timings object.
    // time [number]

    /// Detailed info about the request.
    request: Request,

    /// Detailed info about the response.
    response: Response,

    /// Info about cache usage.
    cache: Cache,

    /// Detailed timing info about request/response round trip.
    timings: Timing,

    /// IP address of the server that was connected (result of DNS resolution).
    server_ip_address: Option<String>,

    /// Unique ID of the parent TCP/IP connection, can be the client port number.
    ///
    /// Note that a port number doesn't have to be unique identifier in cases where the port is
    /// shared for more connections. If the port isn't available for the application, any other
    /// unique connection ID can be used instead (e.g. connection index). Leave out this field if
    /// the application doesn't support this info.
    connection: Option<String>,

    /// A comment provided by the user or the application.
    comment: Option<String>
}

impl <S: Encoder<E>, E> Encodable<S, E> for Entry {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        use OptionalTiming::{TimedContent,NotApplicable};
        encoder.emit_struct("Entry", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            match self.pageref {
                Some(ref pageref) => fields.push(("pageref", Box::new(pageref.to_string()))),
                None => ()
            }
            fields.push(("startedDateTime", Box::new(self.started_date_time.to_string())));
            let mut time = (self.timings.send + self.timings.wait + self.timings.receive) as uint;
            for timing in vec![self.timings.blocked,
                               self.timings.dns,
                               self.timings.connect,
                               self.timings.ssl].iter() {
                time += match *timing {
                    TimedContent(time) => time,
                    NotApplicable => 0u
                }
            }
            fields.push(("time", Box::new(time)));
            fields.push(("request", Box::new(&self.request)));
            fields.push(("response", Box::new(&self.response)));
            fields.push(("cache", Box::new(&self.cache)));
            fields.push(("timings", Box::new(&self.timings)));
            match self.server_ip_address {
                Some(ref server_ip_address) =>
                    fields.push(("serverIPAddress", Box::new(server_ip_address.to_string()))),
                None => ()
            }
            match self.connection {
                Some(ref connection) =>
                    fields.push(("connection", Box::new(connection.to_string()))),
                None => ()
            }
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}

/// This object contains detailed info about performed request.
pub struct Request {
    /// Request method (GET, POST, ...).
    method: String,

    /// Absolute URL of the request (fragments are not included).
    url: String,

    /// Request HTTP Version.
    http_version: String,

    /// List of cookie objects.
    cookies: Vec<Cookie>,

    /// List of header objects.
    headers: Vec<Header>,

    /// List of query parameter objects.
    query_string: Vec<QueryStringPair>,

    /// Posted data info.
    post_data: Option<PostData>,

    /// Total number of bytes from the start of the HTTP request message until (and including)
    /// the double CRLF before the body.
    /// Set to -1 if the info is not available.
    headers_size: Option<int>,

    /// Size of the request body (POST data payload) in bytes.
    /// Set to -1 if the info is not available.
    body_size: Option<int>,

    /// A comment provided by the user or the application.
    comment: Option<String>
}

impl <S: Encoder<E>, E> Encodable<S, E> for Request {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        encoder.emit_struct("Request", 0, |encoder| {
            let default_int = -1i;
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            fields.push(("method", Box::new(self.method.to_string())));
            fields.push(("url", Box::new(self.url.to_string())));
            fields.push(("httpVersion", Box::new(self.http_version.to_string())));
            fields.push(("cookies", Box::new(self.cookies.as_slice())));
            fields.push(("headers", Box::new(self.headers.as_slice())));
            fields.push(("queryString", Box::new(self.query_string.as_slice())));
            match self.post_data {
                Some(ref post_data) => fields.push(("postData", Box::new(post_data))),
                None => ()
            }
            fields.push(("headersSize", Box::new(self.headers_size.unwrap_or(default_int))));
            fields.push(("bodySize", Box::new(self.body_size.unwrap_or(default_int))));
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}

/// This object contains detailed info about the response.
pub struct Response {
    /// Response status.
    status: int,

    /// Response status description.
    status_text: String,

    /// Response HTTP Version.
    http_version: String,

    /// List of cookie objects.
    cookies: Vec<Cookie>,

    /// List of header objects.
    headers: Vec<Header>,

    /// Details about the response body.
    content: Content,

    /// Redirection target URL from the Location response header.
    redirect_url: String,

    /// Total number of bytes from the start of the HTTP response message until (and including) the
    /// double CRLF before the body.
    /// Set to -1 if the info is not available.
    /// The size of received response-headers is computed only from headers that are really
    /// received from the server. Additional headers appended by the browser are not included in
    /// this number, but they appear in the list of header objects.
    headers_size: Option<int>,

    /// Size of the received response body in bytes.
    /// Set to zero in case of responses coming from the cache (304).
    /// Set to -1 if the info is not available.
    body_size: Option<int>,

    /// A comment provided by the user or the application.
    comment: Option<String>
}

impl <S: Encoder<E>, E> Encodable<S, E> for Response {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        encoder.emit_struct("Response", 0, |encoder| {
            let default_int = -1i;
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            fields.push(("status", Box::new(self.status)));
            fields.push(("statusText", Box::new(self.status_text.to_string())));
            fields.push(("httpVersion", Box::new(self.http_version.to_string())));
            fields.push(("cookies", Box::new(self.cookies.as_slice())));
            fields.push(("headers", Box::new(self.headers.as_slice())));
            fields.push(("content", Box::new(&self.content)));
            fields.push(("redirectURL", Box::new(self.redirect_url.to_string())));
            fields.push(("headersSize", Box::new(self.headers_size.unwrap_or(default_int))));
            fields.push(("bodySize", Box::new(self.body_size.unwrap_or(default_int))));
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}


/// This object contains list of all cookies (used in <request> and <response> objects).
pub struct Cookie {
    /// The name of the cookie.
    name: String,

    /// The cookie value.
    value: String,

    /// The path pertaining to the cookie.
    path: Option<String>,

    /// The host of the cookie.
    domain: Option<String>,

    /// Cookie expiration time. (ISO 8601).
    expires: Option<String>,

    /// Set to true if the cookie is HTTP only, false otherwise.
    http_only: Option<bool>,

    /// True if the cookie was transmitted over ssl, false otherwise.
    secure: Option<bool>,

    /// A comment provided by the user or the application.
    comment: Option<String>
}

impl <S: Encoder<E>, E> Encodable<S, E> for Cookie {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        encoder.emit_struct("Cookie", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            fields.push(("name", Box::new(self.name.to_string())));
            fields.push(("value", Box::new(self.value.to_string())));
            match self.path {
                Some(ref path) => fields.push(("path", Box::new(path.to_string()))),
                None => ()
            }
            match self.domain {
                Some(ref domain) => fields.push(("domain", Box::new(domain.to_string()))),
                None => ()
            }
            match self.expires {
                Some(ref expires) => fields.push(("expires", Box::new(expires.to_string()))),
                None => ()
            }
            match self.http_only {
                Some(ref http_only) => fields.push(("httpOnly", Box::new(http_only))),
                None => ()
            }
            match self.secure {
                Some(ref secure) => fields.push(("secure", Box::new(secure))),
                None => ()
            }
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}


/// This object contains list of all headers (used in <request> and <response> objects).
pub struct Header {
    name: String,
    value: String,
    comment: Option<String>
}

impl <S: Encoder<E>, E> Encodable<S, E> for Header {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        encoder.emit_struct("Header", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            fields.push(("name", Box::new(self.name.to_string())));
            fields.push(("value", Box::new(self.value.to_string())));
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}


/// This object contains list of all parameters & values parsed from a query string, if any
/// (embedded in <request> object).
/// HAR format expects NVP (name-value pairs) formatting of the query string.
pub struct QueryStringPair {
    name: String,
    value: String,
    comment: Option<String>
}

impl <S: Encoder<E>, E> Encodable<S, E> for QueryStringPair {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        encoder.emit_struct("QueryStringPair", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            fields.push(("name", Box::new(self.name.to_string())));
            fields.push(("value", Box::new(self.value.to_string())));
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}


/// This object describes posted data, if any (embedded in <request> object).
/// Note that text and params fields are mutually exclusive.
pub struct PostData {
    /// Mime type of posted data.
    mime_type: String,

    /// List of posted parameters (in case of URL encoded parameters).
    params: Vec<Param>,

    /// Plain text posted data
    text: String,

    /// A comment provided by the user or the application.
    comment: Option<String>
}

impl <S: Encoder<E>, E> Encodable<S, E> for PostData {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        encoder.emit_struct("PostData", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            fields.push(("mimeType", Box::new(self.mime_type.to_string())));
            fields.push(("params", Box::new(self.params.as_slice())));
            fields.push(("text", Box::new(self.text.to_string())));
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}


/// List of posted parameters, if any (embedded in <postData> object).
pub struct Param {
    /// name of a posted parameter.
    name: String,

    /// value of a posted parameter or content of a posted file.
    value: Option<String>,

    /// name of a posted file.
    file_name: Option<String>,

    /// content type of a posted file.
    content_type: Option<String>,

    /// A comment provided by the user or the application.
    comment: Option<String>,
}

impl <S: Encoder<E>, E> Encodable<S, E> for Param {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        encoder.emit_struct("Param", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            fields.push(("name", Box::new(self.name.to_string())));
            match self.value {
                Some(ref value) => fields.push(("value", Box::new(value.to_string()))),
                None => ()
            }
            match self.file_name {
                Some(ref file_name) => fields.push(("fileName", Box::new(file_name.to_string()))),
                None => ()
            }
            match self.content_type {
                Some(ref content_type) =>
                    fields.push(("contentType", Box::new(content_type.to_string()))),
                None => ()
            }
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}


/// This object describes details about response content (embedded in <response> object).
///
/// Before setting the text field, the HTTP response is decoded (decompressed & unchunked), than
/// trans-coded from its original character set into UTF-8. Additionally, it can be encoded using
/// e.g. base64. Ideally, the application should be able to unencode a base64 blob and get a
/// byte-for-byte identical resource to what the browser operated on.
pub struct Content {
    /// Length of the returned content in bytes.
    /// Should be equal to response.bodySize if there is no compression and bigger when the content
    /// has been compressed.
    size: int,

    /// Number of bytes saved. Leave out this field if the information is not available.
    compression: Option<int>,

    /// MIME type of the response text (value of the Content-Type response header).
    /// The charset attribute of the MIME type is included (if available).
    mime_type: String,

    /// Response body sent from the server or loaded from the browser cache.
    /// This field is populated with textual content only.
    /// The text field is either HTTP decoded text or a encoded (e.g. "base64") representation of
    /// the response body.
    /// Leave out this field if the information is not available.
    text: Option<String>,

    /// Encoding used for response text field e.g "base64".
    /// Leave out this field if the text field is HTTP decoded (decompressed & unchunked),
    /// than trans-coded from its original character set into UTF-8.
    encoding: Option<String>,

    /// A comment provided by the user or the application.
    comment: Option<String>,
}

impl <S: Encoder<E>, E> Encodable<S, E> for Content {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        encoder.emit_struct("Content", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            fields.push(("size", Box::new(self.size)));
            match self.compression {
                Some(ref compression) => fields.push(("compression", Box::new(compression))),
                None => ()
            }
            fields.push(("mimeType", Box::new(self.mime_type.to_string())));
            match self.text {
                Some(ref text) => fields.push(("text", Box::new(text))),
                None => ()
            }
            match self.encoding {
                Some(ref encoding) => fields.push(("encoding", Box::new(encoding))),
                None => ()
            }
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}


/// This objects contains info about a request coming from browser cache.
pub struct Cache {
    /// State of a cache entry before the request.
    /// Leave out this field if the information is not available.
    before_request: CacheState,

    /// State of a cache entry after the request.
    /// Leave out this field if the information is not available.
    after_request: CacheState,

    comment: Option<String>
}

impl <S: Encoder<E>, E> Encodable<S, E> for Cache {

    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        use CacheState::{Absent,Present,Unknown};
        encoder.emit_struct("Cache", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            match self.before_request {
                Absent => fields.push(("beforeRequest", Box::new(None::<CacheEntry>))),
                Present(ref before_request) =>
                    fields.push(("beforeRequest", Box::new(before_request))),
                Unknown => ()
            }
            match self.after_request {
                Absent => fields.push(("afterRequest", Box::new(None::<CacheEntry>))),
                Present(ref after_request) =>
                    fields.push(("afterRequest", Box::new(after_request))),
                Unknown => ()
            }
            match self.comment {
                Some(ref comment) =>
                    fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}


/// The state of a cache entry.
///
/// Can be Absent, Present, or Unknown. When serialized, these result in (respectively) `null`, a
/// CacheEntry value, or no object.
pub enum CacheState {
    Absent,
    Present(CacheEntry),
    Unknown
}

pub struct CacheEntry {
    /// Expiration time of the cache entry.
    expires: Option<String>,

    /// The last time the cache entry was opened.
    last_access: String,

    /// Etag
    e_tag: String,

    /// The number of times the cache entry has been opened.
    hit_count: int,

    /// (new in 1.2) A comment provided by the user or the application.
    comment: Option<String>,
}

impl <S: Encoder<E>, E> Encodable<S, E> for CacheEntry {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        encoder.emit_struct("CacheEntry", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            match self.expires {
                Some(ref expires) => fields.push(("expires", Box::new(expires.to_string()))),
                None => ()
            }
            fields.push(("lastAccess", Box::new(self.last_access.to_string())));
            fields.push(("eTag", Box::new(self.e_tag.to_string())));
            fields.push(("hitCount", Box::new(self.hit_count)));
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}

/// A timing value which may be absent or present
///
/// Defaults to -1 in the absent case.
#[deriving(Copy)]
pub enum OptionalTiming {
    TimedContent(uint),
    NotApplicable
}

impl <S: Encoder<E>, E> Encodable<S, E> for OptionalTiming {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        use OptionalTiming::{TimedContent,NotApplicable};

        let default_int = -1i;
        let value = match *self {
            TimedContent(value) => value as int,
            NotApplicable => default_int
        };
        try!(encoder.emit_int(value));
        Ok(())
    }
}

/// This object describes various phases within request-response round trip. All times are
/// specified in milliseconds.
///
/// The send, wait and receive timings are not optional and must have non-negative values.
///
/// An exporting tool can omit the blocked, dns, connect and ssl, timings on every request if it is
/// unable to provide them. Tools that can provide these timings can set their values to -1 if they
/// don’t apply. For example, connect would be -1 for requests which re-use an existing connection.
///
/// The time value for the request must be equal to the sum of the timings supplied in this section
/// (excluding any -1 values).
///
/// Following must be true in case there are no -1 values (entry is an object in log.entries) :
/// entry.time == entry.timings.blocked + entry.timings.dns +
///     entry.timings.connect + entry.timings.send + entry.timings.wait +
///         entry.timings.receive;
pub struct Timing {
    /// Time spent in a queue waiting for a network connection.
    /// Use -1 if the timing does not apply to the current request.
    blocked: OptionalTiming,

    /// DNS resolution time. The time required to resolve a host name.
    /// Use -1 if the timing does not apply to the current request.
    dns: OptionalTiming,

    /// Time required to create TCP connection.
    /// Use -1 if the timing does not apply to the current request.
    connect: OptionalTiming,

    /// Time required to send HTTP request to the server.
    send: uint,

    /// Waiting for a response from the server.
    wait: uint,

    /// Time required to read entire response from the server (or cache).
    receive: uint,

    /// Time required for SSL/TLS negotiation.
    /// If this field is defined then the time is also included in the connect field (to ensure
    /// backward compatibility with HAR 1.1).
    /// Use -1 if the timing does not apply to the current request.
    ssl: OptionalTiming,

    /// (new in 1.2) - A comment provided by the user or the application.
    comment: Option<String>
}

impl <S: Encoder<E>, E> Encodable<S, E> for Timing {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        encoder.emit_struct("Timing", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable<S, E>>)> = Vec::new();
            fields.push(("blocked", Box::new(self.blocked)));
            fields.push(("dns", Box::new(self.dns)));
            fields.push(("connect", Box::new(self.connect)));
            fields.push(("send", Box::new(self.send)));
            fields.push(("wait", Box::new(self.wait)));
            fields.push(("receive", Box::new(self.receive)));
            fields.push(("ssl", Box::new(self.ssl)));
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
    }
}

#[cfg(test)]
mod test {
    use serialize::json;
    use Browser;
    use Cache;
    use CacheState::{Absent,Present,Unknown};
    use CacheEntry;
    use Content;
    use Cookie;
    use Creator;
    use Entry;
    use Header;
    use Log;
    use OptionalTiming::{TimedContent,NotApplicable};
    use Page;
    use PageTimings;
    use Param;
    use PostData;
    use QueryStringPair;
    use Request;
    use Response;
    use Timing;

    #[test]
    fn test_log() {
        let mut log = Log::new(
            Some(Browser::new("Firefox".to_string(), "3.6".to_string(), None)),
            Some("Comment".to_string())
        );
        log.add_page(Page::new(
            "2009-04-16T12:07:25.123+01:00".to_string(),
            "page_0".to_string(),
            "Test Page".to_string(),
            PageTimings::new(NotApplicable, NotApplicable, None),
            None
        ));
        log.add_entry(Entry {
            pageref: Some("page_0".to_string()),
            started_date_time: "2009-04-16T12:07:23.596Z".to_string(),
            request: Request {
                method: "GET".to_string(),
                url: "http://www.example.com/path/?param=value".to_string(),
                http_version: "HTTP/1.1".to_string(),
                cookies: Vec::new(),
                headers: Vec::new(),
                query_string: Vec::new(),
                post_data: None,
                headers_size: None,
                body_size: None,
                comment: None,
            },
            response: Response {
                status: 200,
                status_text: "OK".to_string(),
                http_version: "HTTP/1.1".to_string(),
                cookies: Vec::new(),
                headers: Vec::new(),
                content: Content {
                    size: 100,
                    compression: None,
                    mime_type: "text/html; charset=utf8".to_string(),
                    text: None,
                    encoding: None,
                    comment: None
                },
                redirect_url: "".to_string(),
                headers_size: None,
                body_size: None,
                comment: None,
            },
            cache: Cache {
                before_request: Unknown,
                after_request: Unknown,
                comment: None
            },
            timings: Timing {
                blocked: NotApplicable,
                dns: NotApplicable,
                connect: NotApplicable,
                send: 4,
                wait: 5,
                receive: 6,
                ssl: NotApplicable,
                comment: None,
            },
            server_ip_address: None,
            connection: None,
            comment: None
        });
        let log_json = "{
                            \"version\": \"1.2\",
                            \"creator\": {
                                \"name\": \"Rust-HAR\",
                                \"version\": \"0.0.4\"
                            },
                            \"browser\": {
                                \"name\": \"Firefox\",
                                \"version\": \"3.6\"
                            },
                            \"pages\": [
                                {
                                    \"startedDateTime\": \"2009-04-16T12:07:25.123+01:00\",
                                    \"id\": \"page_0\",
                                    \"title\": \"Test Page\",
                                    \"pageTimings\": {
                                        \"onContentLoad\": -1,
                                        \"onLoad\": -1
                                    }
                                }
                            ],
                            \"entries\": [
                                {
                                    \"pageref\": \"page_0\",
                                    \"startedDateTime\": \"2009-04-16T12:07:23.596Z\",
                                    \"request\": {
                                        \"method\": \"GET\",
                                        \"url\": \"http://www.example.com/path/?param=value\",
                                        \"httpVersion\": \"HTTP/1.1\",
                                        \"cookies\": [],
                                        \"headers\": [],
                                        \"queryString\": [],
                                        \"headersSize\": -1,
                                        \"bodySize\": -1
                                    },
                                    \"response\": {
                                        \"status\": 200,
                                        \"statusText\": \"OK\",
                                        \"httpVersion\": \"HTTP/1.1\",
                                        \"cookies\": [],
                                        \"headers\": [],
                                        \"content\": {
                                            \"size\": 100,
                                            \"mimeType\": \"text/html; charset=utf8\"
                                        },
                                        \"redirectURL\": \"\",
                                        \"headersSize\": -1,
                                        \"bodySize\": -1
                                    },
                                    \"cache\": {},
                                    \"time\": 15,
                                    \"timings\": {
                                         \"blocked\": -1,
                                         \"dns\": -1,
                                         \"connect\": -1,
                                         \"send\": 4,
                                         \"wait\": 5,
                                         \"receive\": 6,
                                         \"ssl\": -1
                                    }
                                }
                            ],
                            \"comment\": \"Comment\"
                        }";
        assert_eq!(json::from_str(log_json),
                   json::from_str(json::encode(&log).as_slice()));
    }

    #[test]
    fn test_log_no_optional() {
        let log = Log::new(None, None);
        let log_json = "{
                            \"version\": \"1.2\",
                            \"creator\": {
                                \"name\": \"Rust-HAR\",
                                \"version\": \"0.0.4\"
                            },
                            \"entries\": []
                        }";
        assert_eq!(json::from_str(log_json),
                   json::from_str(json::encode(&log).as_slice()));
    }

    #[test]
    fn test_creator() {
        let creator = Creator {
            name: "Firebug".to_string(),
            version: "1.6".to_string(),
            comment: Some("Comment".to_string())
        };
        let creator_json = "{
                                \"name\": \"Firebug\",
                                \"version\": \"1.6\",
                                \"comment\": \"Comment\"
                            }";
        assert_eq!(json::from_str(creator_json),
                   json::from_str(json::encode(&creator).as_slice()));
    }

    #[test]
    fn test_creator_no_optional() {
        let creator = Creator {
            name: "Firebug".to_string(),
            version: "1.6".to_string(),
            comment: None
        };
        let creator_json = "{
                                \"name\": \"Firebug\",
                                \"version\": \"1.6\"
                            }";
        assert_eq!(json::from_str(creator_json),
                   json::from_str(json::encode(&creator).as_slice()));
    }

    #[test]
    fn test_browser() {
        let browser = Browser::new("Firefox".to_string(), "3.6".to_string(),
                                   Some("Comment".to_string()));
        let browser_json = "{
                                \"name\": \"Firefox\",
                                \"version\": \"3.6\",
                                \"comment\": \"Comment\"
                            }";
        assert_eq!(json::from_str(browser_json),
                   json::from_str(json::encode(&browser).as_slice()));
    }

    #[test]
    fn test_browser_no_optional() {
        let browser = Browser::new("Firefox".to_string(), "3.6".to_string(), None);
        let browser_json = "{
                                \"name\": \"Firefox\",
                                \"version\": \"3.6\"
                            }";
        assert_eq!(json::from_str(browser_json),
                   json::from_str(json::encode(&browser).as_slice()));
    }

    #[test]
    fn test_page() {
        let page = Page::new(
            "2009-04-16T12:07:25.123+01:00".to_string(),
            "page_0".to_string(),
            "Test Page".to_string(),
            PageTimings::new(NotApplicable, NotApplicable, None),
            Some("Comment".to_string())
        );
        let page_json = "{
                             \"startedDateTime\": \"2009-04-16T12:07:25.123+01:00\",
                             \"id\": \"page_0\",
                             \"title\": \"Test Page\",
                             \"pageTimings\": {
                                 \"onContentLoad\": -1,
                                 \"onLoad\": -1
                             },
                             \"comment\": \"Comment\"
                         }";
        assert_eq!(json::from_str(page_json),
                   json::from_str(json::encode(&page).as_slice()));
    }

    #[test]
    fn test_page_no_optional() {
        let page = Page::new(
            "2009-04-16T12:07:25.123+01:00".to_string(),
            "page_0".to_string(),
            "Test Page".to_string(),
            PageTimings::new(NotApplicable, NotApplicable, None),
            None
        );
        let page_json = "{
                             \"startedDateTime\": \"2009-04-16T12:07:25.123+01:00\",
                             \"id\": \"page_0\",
                             \"title\": \"Test Page\",
                             \"pageTimings\": {
                                 \"onContentLoad\": -1,
                                 \"onLoad\": -1
                             }
                         }";
        assert_eq!(json::from_str(page_json),
                   json::from_str(json::encode(&page).as_slice()));
    }

    #[test]
    fn test_page_timings() {
        let page_timings = PageTimings::new(TimedContent(1720),
                                            TimedContent(2500),
                                            Some("Comment".to_string()));
        let page_timings_json = "{
                                     \"onContentLoad\": 1720,
                                     \"onLoad\": 2500,
                                     \"comment\": \"Comment\"
                                 }";
        assert_eq!(json::from_str(page_timings_json),
                   json::from_str(json::encode(&page_timings).as_slice()));
    }

    #[test]
    fn test_page_timings_no_optional() {
        let page_timings = PageTimings::new(NotApplicable, NotApplicable, None);
        let page_timings_json = "{
                                     \"onContentLoad\": -1,
                                     \"onLoad\": -1
                                 }";
        assert_eq!(json::from_str(page_timings_json),
                   json::from_str(json::encode(&page_timings).as_slice()));
    }

    #[test]
    fn test_entry() {
        let entry = Entry {
            pageref: Some("page_0".to_string()),
            started_date_time: "2009-04-16T12:07:23.596Z".to_string(),
            request: Request {
                method: "GET".to_string(),
                url: "http://www.example.com/path/?param=value".to_string(),
                http_version: "HTTP/1.1".to_string(),
                cookies: Vec::new(),
                headers: Vec::new(),
                query_string: Vec::new(),
                post_data: None,
                headers_size: None,
                body_size: None,
                comment: None,
            },
            response: Response {
                status: 200,
                status_text: "OK".to_string(),
                http_version: "HTTP/1.1".to_string(),
                cookies: Vec::new(),
                headers: Vec::new(),
                content: Content {
                    size: 100,
                    compression: None,
                    mime_type: "text/html; charset=utf8".to_string(),
                    text: None,
                    encoding: None,
                    comment: None
                },
                redirect_url: "".to_string(),
                headers_size: None,
                body_size: None,
                comment: None,
            },
            cache: Cache {
                before_request: Unknown,
                after_request: Unknown,
                comment: None
            },
            timings: Timing {
                blocked: TimedContent(1),
                dns: TimedContent(2),
                connect: TimedContent(3),
                send: 4,
                wait: 5,
                receive: 6,
                ssl: TimedContent(7),
                comment: None,
            },
            server_ip_address: Some("10.0.0.1".to_string()),
            connection: Some("52492".to_string()),
            comment: Some("Comment".to_string())
        };
        let entry_json = "{
                              \"pageref\": \"page_0\",
                              \"startedDateTime\": \"2009-04-16T12:07:23.596Z\",
                              \"request\": {
                                  \"method\": \"GET\",
                                  \"url\": \"http://www.example.com/path/?param=value\",
                                  \"httpVersion\": \"HTTP/1.1\",
                                  \"cookies\": [],
                                  \"headers\": [],
                                  \"queryString\": [],
                                  \"headersSize\": -1,
                                  \"bodySize\": -1
                              },
                              \"response\": {
                                  \"status\": 200,
                                  \"statusText\": \"OK\",
                                  \"httpVersion\": \"HTTP/1.1\",
                                  \"cookies\": [],
                                  \"headers\": [],
                                  \"content\": {
                                      \"size\": 100,
                                      \"mimeType\": \"text/html; charset=utf8\"
                                  },
                                  \"redirectURL\": \"\",
                                  \"headersSize\": -1,
                                  \"bodySize\": -1
                              },
                              \"cache\": {},
                              \"time\": 28,
                              \"timings\": {
                                   \"blocked\": 1,
                                   \"dns\": 2,
                                   \"connect\": 3,
                                   \"send\": 4,
                                   \"wait\": 5,
                                   \"receive\": 6,
                                   \"ssl\": 7
                              },
                              \"serverIPAddress\": \"10.0.0.1\",
                              \"connection\": \"52492\",
                              \"comment\": \"Comment\"
                          }";
        assert_eq!(json::from_str(entry_json),
                   json::from_str(json::encode(&entry).as_slice()));
    }

    #[test]
    fn test_entry_no_optional() {
        let entry = Entry {
            pageref: None,
            started_date_time: "2009-04-16T12:07:23.596Z".to_string(),
            request: Request {
                method: "GET".to_string(),
                url: "http://www.example.com/path/?param=value".to_string(),
                http_version: "HTTP/1.1".to_string(),
                cookies: Vec::new(),
                headers: Vec::new(),
                query_string: Vec::new(),
                post_data: None,
                headers_size: None,
                body_size: None,
                comment: None,
            },
            response: Response {
                status: 200,
                status_text: "OK".to_string(),
                http_version: "HTTP/1.1".to_string(),
                cookies: Vec::new(),
                headers: Vec::new(),
                content: Content {
                    size: 100,
                    compression: None,
                    mime_type: "text/html; charset=utf8".to_string(),
                    text: None,
                    encoding: None,
                    comment: None
                },
                redirect_url: "".to_string(),
                headers_size: None,
                body_size: None,
                comment: None,
            },
            cache: Cache {
                before_request: Unknown,
                after_request: Unknown,
                comment: None
            },
            timings: Timing {
                blocked: NotApplicable,
                dns: NotApplicable,
                connect: NotApplicable,
                send: 4,
                wait: 5,
                receive: 6,
                ssl: NotApplicable,
                comment: None,
            },
            server_ip_address: None,
            connection: None,
            comment: None
        };
        let entry_json = "{
                              \"startedDateTime\": \"2009-04-16T12:07:23.596Z\",
                              \"request\": {
                                  \"method\": \"GET\",
                                  \"url\": \"http://www.example.com/path/?param=value\",
                                  \"httpVersion\": \"HTTP/1.1\",
                                  \"cookies\": [],
                                  \"headers\": [],
                                  \"queryString\": [],
                                  \"headersSize\": -1,
                                  \"bodySize\": -1
                              },
                              \"response\": {
                                  \"status\": 200,
                                  \"statusText\": \"OK\",
                                  \"httpVersion\": \"HTTP/1.1\",
                                  \"cookies\": [],
                                  \"headers\": [],
                                  \"content\": {
                                      \"size\": 100,
                                      \"mimeType\": \"text/html; charset=utf8\"
                                  },
                                  \"redirectURL\": \"\",
                                  \"headersSize\": -1,
                                  \"bodySize\": -1
                              },
                              \"cache\": {},
                              \"time\": 15,
                              \"timings\": {
                                   \"blocked\": -1,
                                   \"dns\": -1,
                                   \"connect\": -1,
                                   \"send\": 4,
                                   \"wait\": 5,
                                   \"receive\": 6,
                                   \"ssl\": -1
                              }
                          }";
        assert_eq!(json::from_str(entry_json),
                   json::from_str(json::encode(&entry).as_slice()));
    }

    #[test]
    fn test_request() {
        let request = Request {
            method: "GET".to_string(),
            url: "http://www.example.com/path/?param=value".to_string(),
            http_version: "HTTP/1.1".to_string(),
            cookies: vec![ Cookie {
                name: "TestCookie".to_string(),
                value: "Cookie Value".to_string(),
                path: None,
                domain: None,
                expires: None,
                http_only: None,
                secure: None,
                comment: None
            }],
            headers: vec![ Header {
                name: "Accept-Encoding".to_string(),
                value: "gzip,deflate".to_string(),
                comment: None
            }],
            query_string: vec![QueryStringPair {
                name: "param1".to_string(),
                value: "value1".to_string(),
                comment: None
            }],
            post_data: Some(PostData {
                mime_type: "multipart/form-data".to_string(),
                params: Vec::new(),
                text: "plain posted data".to_string(),
                comment: None
            }),
            headers_size: Some(150),
            body_size: Some(0),
            comment: Some("Comment".to_string())
        };
        let request_json = "{
                                \"method\": \"GET\",
                                \"url\": \"http://www.example.com/path/?param=value\",
                                \"httpVersion\": \"HTTP/1.1\",
                                \"cookies\": [{
                                    \"name\": \"TestCookie\",
                                    \"value\": \"Cookie Value\"
                                }],
                                \"headers\": [
                                    {
                                        \"name\": \"Accept-Encoding\",
                                        \"value\": \"gzip,deflate\"
                                    }
                                ],
                                \"queryString\": [
                                    {
                                          \"name\": \"param1\",
                                          \"value\": \"value1\"
                                    }
                                ],
                                \"postData\": {
                                    \"mimeType\": \"multipart/form-data\",
                                    \"params\": [],
                                    \"text\": \"plain posted data\"
                                },
                                \"headersSize\": 150,
                                \"bodySize\": 0,
                                \"comment\": \"Comment\"
                            }";
        assert_eq!(json::from_str(request_json),
                   json::from_str(json::encode(&request).as_slice()));
    }

    #[test]
    fn test_request_no_optional() {
        let request = Request {
            method: "GET".to_string(),
            url: "http://www.example.com/path/?param=value".to_string(),
            http_version: "HTTP/1.1".to_string(),
            cookies: Vec::new(),
            headers: Vec::new(),
            query_string: Vec::new(),
            post_data: None,
            headers_size: None,
            body_size: None,
            comment: None,
        };
        let request_json = "{
                                \"method\": \"GET\",
                                \"url\": \"http://www.example.com/path/?param=value\",
                                \"httpVersion\": \"HTTP/1.1\",
                                \"cookies\": [],
                                \"headers\": [],
                                \"queryString\": [],
                                \"headersSize\": -1,
                                \"bodySize\": -1
                            }";
        assert_eq!(json::from_str(request_json),
                   json::from_str(json::encode(&request).as_slice()));
    }

    #[test]
    fn test_response() {
        let response = Response {
            status: 200,
            status_text: "OK".to_string(),
            http_version: "HTTP/1.1".to_string(),
            cookies: Vec::new(),
            headers: Vec::new(),
            content: Content {
                size: 100,
                compression: None,
                mime_type: "text/html; charset=utf8".to_string(),
                text: None,
                encoding: None,
                comment: None
            },
            redirect_url: "".to_string(),
            headers_size: Some(160),
            body_size: Some(850),
            comment: Some("".to_string()),
        };
        let response_json = "{
                                \"status\": 200,
                                \"statusText\": \"OK\",
                                \"httpVersion\": \"HTTP/1.1\",
                                \"cookies\": [],
                                \"headers\": [],
                                \"content\": {
                                    \"size\": 100,
                                    \"mimeType\": \"text/html; charset=utf8\"
                                },
                                \"redirectURL\": \"\",
                                \"headersSize\" : 160,
                                \"bodySize\" : 850,
                                \"comment\" : \"\"
                            }";
        assert_eq!(json::from_str(response_json),
                   json::from_str(json::encode(&response).as_slice()));
    }

    #[test]
    fn test_response_no_optional() {
        let response = Response {
            status: 200,
            status_text: "OK".to_string(),
            http_version: "HTTP/1.1".to_string(),
            cookies: Vec::new(),
            headers: Vec::new(),
            content: Content {
                size: 100,
                compression: None,
                mime_type: "text/html; charset=utf8".to_string(),
                text: None,
                encoding: None,
                comment: None
            },
            redirect_url: "".to_string(),
            headers_size: None,
            body_size: None,
            comment: None,
        };
        let response_json = "{
                                \"status\": 200,
                                \"statusText\": \"OK\",
                                \"httpVersion\": \"HTTP/1.1\",
                                \"cookies\": [],
                                \"headers\": [],
                                \"content\": {
                                    \"size\": 100,
                                    \"mimeType\": \"text/html; charset=utf8\"
                                },
                                \"redirectURL\": \"\",
                                \"headersSize\" : -1,
                                \"bodySize\" : -1
                            }";
        assert_eq!(json::from_str(response_json),
                   json::from_str(json::encode(&response).as_slice()));
    }

    #[test]
    fn test_cookie() {
        let cookie = Cookie {
            name: "TestCookie".to_string(),
            value: "Cookie Value".to_string(),
            path: Some("/".to_string()),
            domain: Some("www.janodvarko.cz".to_string()),
            expires: Some("2009-07-24T19:20:30.123+02:00".to_string()),
            http_only: Some(false),
            secure: Some(false),
            comment: Some("".to_string()),
        };
        let cookie_json = "{
                               \"name\": \"TestCookie\",
                               \"value\": \"Cookie Value\",
                               \"path\": \"/\",
                               \"domain\": \"www.janodvarko.cz\",
                               \"expires\": \"2009-07-24T19:20:30.123+02:00\",
                               \"httpOnly\": false,
                               \"secure\": false,
                               \"comment\": \"\"
                           }";
        assert_eq!(json::from_str(cookie_json),
                   json::from_str(json::encode(&cookie).as_slice()));
    }

    #[test]
    fn test_cookie_no_optional() {
        let cookie = Cookie {
            name: "TestCookie".to_string(),
            value: "Cookie Value".to_string(),
            path: None,
            domain: None,
            expires: None,
            http_only: None,
            secure: None,
            comment: None
        };
        let cookie_json = "{
                               \"name\": \"TestCookie\",
                               \"value\": \"Cookie Value\"
                           }";
        assert_eq!(json::from_str(cookie_json),
                   json::from_str(json::encode(&cookie).as_slice()));
    }

    #[test]
    fn test_header() {
        let header = Header {
            name: "Accept-Encoding".to_string(),
            value: "gzip,deflate".to_string(),
            comment: Some("Comment".to_string())
        };
        let header_json = "{
                               \"name\": \"Accept-Encoding\",
                               \"value\": \"gzip,deflate\",
                               \"comment\": \"Comment\"
                           }";
        assert_eq!(json::from_str(header_json),
                   json::from_str(json::encode(&header).as_slice()));
    }

    #[test]
    fn test_header_no_optional() {
        let header = Header {
            name: "Accept-Encoding".to_string(),
            value: "gzip,deflate".to_string(),
            comment: None
        };
        let header_json = "{
                               \"name\": \"Accept-Encoding\",
                               \"value\": \"gzip,deflate\"
                           }";
        assert_eq!(json::from_str(header_json),
                   json::from_str(json::encode(&header).as_slice()));
    }

    #[test]
    fn test_query_string_pair() {
        let query_string_pair = QueryStringPair {
            name: "param1".to_string(),
            value: "value1".to_string(),
            comment: Some("Comment".to_string())
        };
        let query_string_pair_json = "{
                                          \"name\": \"param1\",
                                          \"value\": \"value1\",
                                          \"comment\": \"Comment\"
                                      }";
        assert_eq!(json::from_str(query_string_pair_json),
                   json::from_str(json::encode(&query_string_pair).as_slice()));
    }

    #[test]
    fn test_query_string_pair_no_optional() {
        let query_string_pair = QueryStringPair {
            name: "param1".to_string(),
            value: "value1".to_string(),
            comment: None
        };
        let query_string_pair_json = "{
                                          \"name\": \"param1\",
                                          \"value\": \"value1\"
                                      }";
        assert_eq!(json::from_str(query_string_pair_json),
                   json::from_str(json::encode(&query_string_pair).as_slice()));
    }

    #[test]
    fn test_post_data() {
        let post_data = PostData {
            mime_type: "multipart/form-data".to_string(),
            params: vec![Param {
                name: "paramName".to_string(),
                value: None,
                file_name: None,
                content_type: None,
                comment: None
            }],
            text: "plain posted data".to_string(),
            comment: Some("Comment".to_string())
        };
        let post_data_json = "{
                                  \"mimeType\": \"multipart/form-data\",
                                  \"params\": [
                                      {
                                          \"name\": \"paramName\"
                                      }
                                  ],
                                  \"text\": \"plain posted data\",
                                  \"comment\": \"Comment\"
                              }";
        assert_eq!(json::from_str(post_data_json),
                   json::from_str(json::encode(&post_data).as_slice()));
    }

    #[test]
    fn test_post_data_no_optional() {
        let post_data = PostData {
            mime_type: "multipart/form-data".to_string(),
            params: Vec::new(),
            text: "plain posted data".to_string(),
            comment: None
        };
        let post_data_json = "{
                                  \"mimeType\": \"multipart/form-data\",
                                  \"params\": [],
                                  \"text\": \"plain posted data\"
                              }";
        assert_eq!(json::from_str(post_data_json),
                   json::from_str(json::encode(&post_data).as_slice()));
    }

    #[test]
    fn test_param() {
        let param = Param {
            name: "paramName".to_string(),
            value: Some("paramValue".to_string()),
            file_name: Some("example.pdf".to_string()),
            content_type: Some("application/pdf".to_string()),
            comment: Some("Comment".to_string())
        };
        let param_json = "{
                              \"name\": \"paramName\",
                              \"value\": \"paramValue\",
                              \"fileName\": \"example.pdf\",
                              \"contentType\": \"application/pdf\",
                              \"comment\": \"Comment\"
                          }";
        assert_eq!(json::from_str(param_json),
                   json::from_str(json::encode(&param).as_slice()));
    }

    #[test]
    fn test_param_no_optional() {
        let param = Param {
            name: "paramName".to_string(),
            value: None,
            file_name: None,
            content_type: None,
            comment: None
        };
        let param_json = "{
                              \"name\": \"paramName\"
                          }";
        assert_eq!(json::from_str(param_json),
                   json::from_str(json::encode(&param).as_slice()));
    }

    #[test]
    fn test_content() {
        let content = Content {
            size: 100,
            compression: Some(0),
            mime_type: "text/html; charset=utf8".to_string(),
            text: Some("\n".to_string()),
            encoding: Some("base64".to_string()),
            comment: Some("Comment".to_string())
        };
        let content_json = "{
                                \"size\": 100,
                                \"compression\": 0,
                                \"mimeType\": \"text/html; charset=utf8\",
                                \"text\": \"\\n\",
                                \"encoding\": \"base64\",
                                \"comment\": \"Comment\"
                            }";
        assert_eq!(json::from_str(content_json),
                   json::from_str(json::encode(&content).as_slice()));
    }

    #[test]
    fn test_content_no_optional() {
        let content = Content {
            size: 100,
            compression: None,
            mime_type: "text/html; charset=utf8".to_string(),
            text: None,
            encoding: None,
            comment: None
        };
        let content_json = "{
                                \"size\": 100,
                                \"mimeType\": \"text/html; charset=utf8\"
                            }";
        assert_eq!(json::from_str(content_json),
                   json::from_str(json::encode(&content).as_slice()));
    }

    #[test]
    fn test_cache() {
        let cache = Cache {
            before_request: Present(CacheEntry {
                expires: None,
                last_access: "2000-01-01T00:00:00.000Z".to_string(),
                e_tag: "123456789".to_string(),
                hit_count: 42,
                comment: None
            }),
            after_request: Present(CacheEntry {
                expires: None,
                last_access: "2000-02-01T00:00:00.000Z".to_string(),
                e_tag: "987654321".to_string(),
                hit_count: 24,
                comment: None
            }),
            comment: Some("Comment".to_string())
        };
        let cache_json = "{
                              \"beforeRequest\": {
                                  \"lastAccess\": \"2000-01-01T00:00:00.000Z\",
                                  \"eTag\": \"123456789\",
                                  \"hitCount\": 42
                              },
                              \"afterRequest\": {
                                  \"lastAccess\": \"2000-02-01T00:00:00.000Z\",
                                  \"eTag\": \"987654321\",
                                  \"hitCount\": 24
                              },
                              \"comment\": \"Comment\"
                          }";
        assert_eq!(json::from_str(cache_json),
                   json::from_str(json::encode(&cache).as_slice()));
    }

    #[test]
    fn test_cache_absent_entries() {
        let cache = Cache {
            before_request: Absent,
            after_request: Absent,
            comment: None
        };
        let cache_json = "{
                              \"beforeRequest\": null,
                              \"afterRequest\": null
                          }";
        assert_eq!(json::from_str(cache_json),
                   json::from_str(json::encode(&cache).as_slice()));
    }

    #[test]
    fn test_cache_unknown_entries() {
        let cache = Cache {
            before_request: Unknown,
            after_request: Unknown,
            comment: None
        };
        let cache_json = "{}";
        assert_eq!(json::from_str(cache_json),
                   json::from_str(json::encode(&cache).as_slice()));
    }


    #[test]
    fn test_cache_entry() {
        let cache_entry = CacheEntry {
            expires: Some("2000-02-01T00:00:00.000Z".to_string()),
            last_access: "2000-01-01T00:00:00.000Z".to_string(),
            e_tag: "123456789".to_string(),
            hit_count: 42,
            comment: Some("Comment".to_string())
        };
        let cache_entry_json = "{
                                    \"expires\": \"2000-02-01T00:00:00.000Z\",
                                    \"lastAccess\": \"2000-01-01T00:00:00.000Z\",
                                    \"eTag\": \"123456789\",
                                    \"hitCount\": 42,
                                    \"comment\": \"Comment\"
                                }";
        assert_eq!(json::from_str(cache_entry_json),
                   json::from_str(json::encode(&cache_entry).as_slice()));
    }

    #[test]
    fn test_cache_entry_no_optional() {
        let cache_entry = CacheEntry {
            expires: None,
            last_access: "2000-01-01T00:00:00.000Z".to_string(),
            e_tag: "123456789".to_string(),
            hit_count: 42,
            comment: None
        };
        let cache_entry_json = "{
                                    \"lastAccess\": \"2000-01-01T00:00:00.000Z\",
                                    \"eTag\": \"123456789\",
                                    \"hitCount\": 42
                                }";
        assert_eq!(json::from_str(cache_entry_json),
                   json::from_str(json::encode(&cache_entry).as_slice()));
    }
    #[test]
    fn test_timing() {
        let timing = Timing {
            blocked: TimedContent(1),
            dns: TimedContent(2),
            connect: TimedContent(3),
            send: 4,
            wait: 5,
            receive: 6,
            ssl: TimedContent(7),
            comment: Some("Comment".to_string()),
        };
        let timing_json = "{
                                \"blocked\": 1,
                                \"dns\": 2,
                                \"connect\": 3,
                                \"send\": 4,
                                \"wait\": 5,
                                \"receive\": 6,
                                \"ssl\": 7,
                                \"comment\":\"Comment\"
                           }";
        assert_eq!(json::from_str(timing_json),
                   json::from_str(json::encode(&timing).as_slice()));
    }

    #[test]
    fn test_timing_no_optional() {
        let timing = Timing {
            blocked: NotApplicable,
            dns: NotApplicable,
            connect: NotApplicable,
            send: 4,
            wait: 5,
            receive: 6,
            ssl: NotApplicable,
            comment: None,
        };
        let timing_json = "{
                                \"blocked\": -1,
                                \"dns\": -1,
                                \"connect\": -1,
                                \"send\": 4,
                                \"wait\": 5,
                                \"receive\": 6,
                                \"ssl\": -1
                           }";
        assert_eq!(json::from_str(timing_json),
                   json::from_str(json::encode(&timing).as_slice()));
    }

}
