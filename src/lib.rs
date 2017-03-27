/// Implements struct hierarchy and serializer for the [HAR 1.2 spec][1].
///
/// [1]: http://www.softwareishard.com/blog/har-12-spec/

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use serde::de::{Deserialize, Deserializer};

const HAR_VERSION: &'static str = "1.2";
const HAR_CREATOR_NAME: &'static str = "Rust-HAR";
const HAR_CREATOR_VERSION: &'static str = "0.0.4";

/// This object represents the root of the exported data.
///
/// This object MUST be present and its name MUST be "log".
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
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
            creator: Creator::new(
                HAR_CREATOR_NAME.to_string(),
                HAR_CREATOR_VERSION.to_string(),
                None
            ),
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

<<<<<<< HEAD
impl Encodable for Log {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("Log", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
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

=======
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
/// This object contains information about the log creator application.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    name: String,
    version: String,
    comment: Option<String>
}

<<<<<<< HEAD
impl Encodable for Creator {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("Creator", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
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
=======
impl Creator {
    pub fn new(name: String, version: String, comment: Option<String>) -> Creator{
        Creator {
            name: name,
            version: version,
            comment: comment
        }
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
    }
}

/// This object contains information about the browser that created the log.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
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

<<<<<<< HEAD
impl Encodable for Browser {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("Browser", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
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


=======
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
/// This object represents list of exported pages.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
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

<<<<<<< HEAD
impl Encodable for Page {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("Page", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
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


=======
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
/// This object describes timings for various events (states) fired during the page load.
///
/// All times are specified in milliseconds.
/// If a time info is not available appropriate field is set to -1.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
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

<<<<<<< HEAD
impl Encodable for PageTimings {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("PageTimings", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
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

=======
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
/// This object represents an array with all exported HTTP requests. Sorting entries by
/// startedDateTime (starting from the oldest) is preferred way how to export data since it can
/// make importing faster. However the reader application should always make sure the array is
/// sorted (if required for the import).
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
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

<<<<<<< HEAD
impl Encodable for Entry {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        use OptionalTiming::{TimedContent,NotApplicable};
        encoder.emit_struct("Entry", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
            match self.pageref {
                Some(ref pageref) => fields.push(("pageref", Box::new(pageref.to_string()))),
                None => ()
            }
            fields.push(("startedDateTime", Box::new(self.started_date_time.to_string())));
            let mut time = (self.timings.send + self.timings.wait + self.timings.receive) as usize;
            for timing in vec![self.timings.blocked,
                               self.timings.dns,
                               self.timings.connect,
                               self.timings.ssl].iter() {
                time += match *timing {
                    TimedContent(time) => time,
                    NotApplicable => 0us
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
=======
impl Entry {
    pub fn new(
        pageref: Option<String>,
        started_date_time: String,
        request: Request,
        response: Response,
        cache: Cache,
        timings: Timing,
        server_ip_address: Option<String>,
        connection: Option<String>,
        comment: Option<String>
    ) -> Entry {
        Entry {
            pageref: pageref,
            started_date_time: started_date_time,
            request: request,
            response: response,
            cache: cache,
            timings: timings,
            server_ip_address: server_ip_address,
            connection: connection,
            comment: comment
        }
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
    }
}

/// This object contains detailed info about performed request.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
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
<<<<<<< HEAD
    headers_size: Option<isize>,

    /// Size of the request body (POST data payload) in bytes.
    /// Set to -1 if the info is not available.
    body_size: Option<isize>,
=======
    headers_size: Option<i32>,

    /// Size of the request body (POST data payload) in bytes.
    /// Set to -1 if the info is not available.
    body_size: Option<i32>,
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc

    /// A comment provided by the user or the application.
    comment: Option<String>
}

<<<<<<< HEAD
impl Encodable for Request {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("Request", 0, |encoder| {
            let default_isize = -1is;
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
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
            fields.push(("headersSize", Box::new(self.headers_size.unwrap_or(default_isize))));
            fields.push(("bodySize", Box::new(self.body_size.unwrap_or(default_isize))));
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
=======
impl Request {
    pub fn new(
        method: String,
        url: String,
        http_version: String,
        cookies: Vec<Cookie>,
        headers: Vec<Header>,
        query_string: Vec<QueryStringPair>,
        post_data: Option<PostData>,
        headers_size: Option<i32>,
        body_size: Option<i32>,
        comment: Option<String>
    ) -> Request {
        Request {
            method: method,
            url: url,
            http_version: http_version,
            cookies: cookies,
            headers: headers,
            query_string: query_string,
            post_data: post_data,
            headers_size: headers_size,
            body_size: body_size,
            comment: comment
        }
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
    }
}

/// This object contains detailed info about the response.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Response status.
<<<<<<< HEAD
    status: isize,
=======
    status: i32,
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc

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
    #[serde(rename = "redirectURL")]
    redirect_url: String,

    /// Total number of bytes from the start of the HTTP response message until (and including) the
    /// double CRLF before the body.
    /// Set to -1 if the info is not available.
    /// The size of received response-headers is computed only from headers that are really
    /// received from the server. Additional headers appended by the browser are not included in
    /// this number, but they appear in the list of header objects.
<<<<<<< HEAD
    headers_size: Option<isize>,
=======
    headers_size: Option<i32>,
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc

    /// Size of the received response body in bytes.
    /// Set to zero in case of responses coming from the cache (304).
    /// Set to -1 if the info is not available.
<<<<<<< HEAD
    body_size: Option<isize>,
=======
    body_size: Option<i32>,
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc

    /// A comment provided by the user or the application.
    comment: Option<String>
}

<<<<<<< HEAD
impl Encodable for Response {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("Response", 0, |encoder| {
            let default_isize = -1is;
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
            fields.push(("status", Box::new(self.status)));
            fields.push(("statusText", Box::new(self.status_text.to_string())));
            fields.push(("httpVersion", Box::new(self.http_version.to_string())));
            fields.push(("cookies", Box::new(self.cookies.as_slice())));
            fields.push(("headers", Box::new(self.headers.as_slice())));
            fields.push(("content", Box::new(&self.content)));
            fields.push(("redirectURL", Box::new(self.redirect_url.to_string())));
            fields.push(("headersSize", Box::new(self.headers_size.unwrap_or(default_isize))));
            fields.push(("bodySize", Box::new(self.body_size.unwrap_or(default_isize))));
            match self.comment {
                Some(ref comment) => fields.push(("comment", Box::new(comment.to_string()))),
                None => ()
            }
            for (idx, &(name, ref value)) in fields.iter().enumerate() {
                try!(encoder.emit_struct_field(name, idx, |encoder| (**value).encode(encoder)));
            }
            Ok(())
        })
=======
impl Response {
    pub fn new(
        status: i32,
        status_text: String,
        http_version: String,
        cookies: Vec<Cookie>,
        headers: Vec<Header>,
        content: Content,
        redirect_url: String,
        headers_size: Option<i32>,
        body_size: Option<i32>,
        comment: Option<String>
    ) -> Response {
        Response {
            status: status,
            status_text: status_text,
            http_version: http_version,
            cookies: cookies,
            headers: headers,
            content: content,
            redirect_url: redirect_url,
            headers_size: headers_size,
            body_size: body_size,
            comment: comment
        }
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
    }
}


/// This object contains list of all cookies (used in <request> and <response> objects).
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
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

<<<<<<< HEAD
impl Encodable for Cookie {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("Cookie", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
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
=======
impl Cookie {
    pub fn new(
        name: String,
        value: String,
        path: Option<String>,
        domain: Option<String>,
        expires: Option<String>,
        http_only: Option<bool>,
        secure: Option<bool>,
        comment: Option<String>
    ) -> Cookie {
        Cookie {
            name: name,
            value: value,
            path: path,
            domain: domain,
            expires: expires,
            http_only: http_only,
            secure: secure,
            comment: comment
        }
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
    }
}


/// This object contains list of all headers (used in <request> and <response> objects).
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    name: String,
    value: String,
    comment: Option<String>
}

<<<<<<< HEAD
impl Encodable for Header {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("Header", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
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
=======
impl Header {
    pub fn new(
        name: String,
        value: String,
        comment: Option<String>
    ) -> Header {
        Header {
            name: name,
            value: value,
            comment: comment
        }
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
    }
}

/// This object contains list of all parameters & values parsed from a query string, if any
/// (embedded in <request> object).
/// HAR format expects NVP (name-value pairs) formatting of the query string.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryStringPair {
    name: String,
    value: String,
    comment: Option<String>
}

<<<<<<< HEAD
impl Encodable for QueryStringPair {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("QueryStringPair", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
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
=======
impl QueryStringPair {
    pub fn new(
        name: String,
        value: String,
        comment: Option<String>
    ) -> QueryStringPair {
        QueryStringPair {
            name: name,
            value: value,
            comment: comment
        }
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
    }
}

/// This object describes posted data, if any (embedded in <request> object).
/// Note that text and params fields are mutually exclusive.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
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

<<<<<<< HEAD
impl Encodable for PostData {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("PostData", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
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
=======
impl PostData {
    pub fn new(
        mime_type: String,
        params: Vec<Param>,
        text: String,
        comment: Option<String>
    ) -> PostData {
        PostData {
            mime_type: mime_type,
            params: params,
            text: text,
            comment: comment
        }
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
    }
}

/// List of posted parameters, if any (embedded in <postData> object).
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
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

<<<<<<< HEAD
impl Encodable for Param {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("Param", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
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
=======
impl Param {
    pub fn new(
        name: String,
        value: Option<String>,
        file_name: Option<String>,
        content_type: Option<String>,
        comment: Option<String>
    ) -> Param {
        Param {
            name: name,
            value: value,
            file_name: file_name,
            content_type: content_type,
            comment: comment
        }
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
    }
}

/// This object describes details about response content (embedded in <response> object).
///
/// Before setting the text field, the HTTP response is decoded (decompressed & unchunked), than
/// trans-coded from its original character set into UTF-8. Additionally, it can be encoded using
/// e.g. base64. Ideally, the application should be able to unencode a base64 blob and get a
/// byte-for-byte identical resource to what the browser operated on.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    /// Length of the returned content in bytes.
    /// Should be equal to response.bodySize if there is no compression and bigger when the content
    /// has been compressed.
<<<<<<< HEAD
    size: isize,

    /// Number of bytes saved. Leave out this field if the information is not available.
    compression: Option<isize>,
=======
    size: i32,

    /// Number of bytes saved. Leave out this field if the information is not available.
    compression: Option<i32>,
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc

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

<<<<<<< HEAD
impl Encodable for Content {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("Content", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
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
=======
impl Content {
    pub fn new(
        size: i32,
        compression: Option<i32>,
        mime_type: String,
        text: Option<String>,
        encoding: Option<String>,
        comment: Option<String>
    ) -> Content {
        Content {
            size: size,
            compression: compression,
            mime_type: mime_type,
            text: text,
            encoding: encoding,
            comment: comment
        }
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
    }
}

/// This objects contains info about a request coming from browser cache.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cache {
    /// State of a cache entry before the request.
    /// Leave out this field if the information is not available.
    #[serde(default = "CacheState::unknown")]
    before_request: CacheState,

    /// State of a cache entry after the request.
    /// Leave out this field if the information is not available.
    #[serde(default = "CacheState::unknown")]
    after_request: CacheState,

    comment: Option<String>
}

<<<<<<< HEAD
impl Encodable for Cache {

    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        use CacheState::{Absent,Present,Unknown};
        encoder.emit_struct("Cache", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
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
=======
impl Cache {
    pub fn new(
        before_request: CacheState,
        after_request: CacheState,
        comment: Option<String>
    ) -> Cache {
        Cache {
            before_request: before_request,
            after_request: after_request,
            comment: comment
        }
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
    }
}

/// The state of a cache entry.
///
/// Can be Absent, Present, or Unknown. When serialized, these result in (respectively) `null`, a
/// CacheEntry value, or no object.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum CacheState {
    Absent,
    Present(CacheEntry),
    Unknown
}

impl CacheState {
    fn unknown() -> Self { CacheState::Unknown }
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CacheEntry {
    /// Expiration time of the cache entry.
    expires: Option<String>,

    /// The last time the cache entry was opened.
    last_access: String,

    /// Etag
    e_tag: String,

    /// The number of times the cache entry has been opened.
<<<<<<< HEAD
    hit_count: isize,
=======
    hit_count: i32,
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc

    /// (new in 1.2) A comment provided by the user or the application.
    comment: Option<String>,
}

<<<<<<< HEAD
impl Encodable for CacheEntry {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("CacheEntry", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
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
=======
impl CacheEntry {
    pub fn new(
        expires: Option<String>,
        last_access: String,
        e_tag: String,
        hit_count: i32,
        comment: Option<String>
    ) -> CacheEntry {
        CacheEntry {
            expires: expires,
            last_access: last_access,
            e_tag: e_tag,
            hit_count: hit_count,
            comment: comment
        }
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
    }
}

/// A timing value which may be absent or present
///
/// Defaults to -1 in the absent case.
#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum OptionalTiming {
<<<<<<< HEAD
    TimedContent(usize),
    NotApplicable
}

impl Encodable for OptionalTiming {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        use OptionalTiming::{TimedContent,NotApplicable};

        let default_isize = -1is;
        let value = match *self {
            TimedContent(value) => value as isize,
            NotApplicable => default_isize
        };
        try!(encoder.emit_isize(value));
        Ok(())
=======
    TimedContent(u32),
    NotApplicable
}

impl Deserialize for OptionalTiming {
    fn deserialize<D>(deserializer: D) -> Result<OptionalTiming, D::Error>
        where D: Deserializer
    {
        let deser_result: serde_json::Value = try!(serde::Deserialize::deserialize(deserializer));
        match deser_result {
            //serde_json::Value::Number(n) => Ok(OptionalTiming::TimedContent(n.as_u64().unwrap() as u32)),
            serde_json::Value::Number(ref n) if n.as_i64().unwrap() >= 0 as i64 => 
	            Ok(OptionalTiming::TimedContent(n.as_u64().unwrap() as u32)),
            serde_json::Value::Number(ref n) if n.as_i64().unwrap() == -1 as i64 => 
	            Ok(OptionalTiming::NotApplicable),
            _ => Err(serde::de::Error::custom("Unexpected value")),
        }
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
    }
}

/// This object describes various phases within request-response round trip. All times are
/// specified in milliseconds.
///
/// The send, wait and receive timings are not optional and must have non-negative values.
///
/// An exporting tool can omit the blocked, dns, connect and ssl, timings on every request if it is
/// unable to provide them. Tools that can provide these timings can set their values to -1 if they
/// don't apply. For example, connect would be -1 for requests which re-use an existing connection.
///
/// The time value for the request must be equal to the sum of the timings supplied in this section
/// (excluding any -1 values).
///
/// Following must be true in case there are no -1 values (entry is an object in log.entries) :
/// entry.time == entry.timings.blocked + entry.timings.dns +
///     entry.timings.connect + entry.timings.send + entry.timings.wait +
///         entry.timings.receive;
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
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
<<<<<<< HEAD
    send: usize,

    /// Waiting for a response from the server.
    wait: usize,

    /// Time required to read entire response from the server (or cache).
    receive: usize,
=======
    send: u32,

    /// Waiting for a response from the server.
    wait: u32,

    /// Time required to read entire response from the server (or cache).
    receive: u32,
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc

    /// Time required for SSL/TLS negotiation.
    /// If this field is defined then the time is also included in the connect field (to ensure
    /// backward compatibility with HAR 1.1).
    /// Use -1 if the timing does not apply to the current request.
    ssl: OptionalTiming,

    /// (new in 1.2) - A comment provided by the user or the application.
    comment: Option<String>
}

<<<<<<< HEAD
impl Encodable for Timing {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("Timing", 0, |encoder| {
            let mut fields: Vec<(&str, Box<Encodable>)> = Vec::new();
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
=======
impl Timing {
    pub fn new(
        blocked: OptionalTiming,
        dns: OptionalTiming,
        connect: OptionalTiming,
        send: u32,
        wait: u32,
        receive: u32,
        ssl: OptionalTiming,
        comment: Option<String>
    ) -> Timing {
        Timing {
            blocked: blocked,
            dns: dns,
            connect: connect,
            send: send,
            wait: wait,
            receive: receive,
            ssl: ssl,
            comment: comment
        }
>>>>>>> 553f592fe4180d0c73d33a610bbe139c23a8b0bc
    }
}


#[cfg(test)]
mod test {
	
	use serde_json;
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
        log.add_entry(Entry::new(
            Some("page_0".to_string()),
            "2009-04-16T12:07:23.596Z".to_string(),
            Request::new(
                "GET".to_string(),
                "http://www.example.com/path/?param=value".to_string(),
                "HTTP/1.1".to_string(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                None,
                None,
                None,
                None
            ),
            Response::new(
                200,
                "OK".to_string(),
                "HTTP/1.1".to_string(),
                Vec::new(),
                Vec::new(),
                Content::new(
                    100,
                    None,
                    "text/html; charset=utf8".to_string(),
                    None,
                    None,
                    None
                ),
                "".to_string(),
                None,
                None,
                None
            ),
            Cache::new(
                Absent,
                Absent,
                None
            ),
            Timing::new(
                NotApplicable,
                NotApplicable,
                NotApplicable,
                4,
                5,
                6,
                NotApplicable,
                None
            ),
            None,
            None,
            None
        ));
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
                                        \"headersSize\": null,
                                        \"bodySize\": null
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
                                        \"headersSize\": null,
                                        \"bodySize\": null
                                    },
                                    \"cache\": {
                                        \"beforeRequest\": null,
                                        \"afterRequest\": null
                                    },
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
        let log_from_str: Log = serde_json::from_str(log_json).unwrap();
        assert_eq!(log_from_str, log);
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
        let log_from_str: Log = serde_json::from_str(log_json).unwrap();
        assert_eq!( log_from_str, log );
    }

    #[test]
    fn test_creator() {
        let creator = Creator::new(
            "Firebug".to_string(),
            "1.6".to_string(),
            Some("Comment".to_string())
        );
        let creator_json = "{
                                \"name\": \"Firebug\",
                                \"version\": \"1.6\",
                                \"comment\": \"Comment\"
                            }";
        let creator_from_str: Creator = serde_json::from_str(creator_json).unwrap();
        assert_eq!( creator_from_str, creator );
    }

    #[test]
    fn test_creator_no_optional() {
        let creator = Creator::new(
            "Firebug".to_string(),
            "1.6".to_string(),
            None
        );

        let creator_json = "{
                                \"name\": \"Firebug\",
                                \"version\": \"1.6\"
                            }";
        let creator_from_str: Creator = serde_json::from_str(creator_json).unwrap();
        assert_eq!( creator_from_str, creator );
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

        let browser_from_str: Browser = serde_json::from_str(browser_json).unwrap();
        assert_eq!( browser_from_str, browser );
    }

    #[test]
    fn test_browser_no_optional() {
        let browser = Browser::new("Firefox".to_string(), "3.6".to_string(), None);
        let browser_json = "{
                                \"name\": \"Firefox\",
                                \"version\": \"3.6\"
                            }";
        let browser_from_str: Browser = serde_json::from_str(browser_json).unwrap();
        assert_eq!( browser_from_str, browser );
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
        let page_from_str: Page = serde_json::from_str(page_json).unwrap();
        assert_eq!( page_from_str, page );
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
        let page_from_str: Page = serde_json::from_str(page_json).unwrap();
        assert_eq!( page_from_str, page );
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
        let page_timings_from_str: PageTimings = serde_json::from_str(page_timings_json).unwrap();
        assert_eq!(page_timings_from_str, page_timings );
    }

    #[test]
    fn test_page_timings_no_optional() {
        let page_timings = PageTimings::new(NotApplicable, NotApplicable, None);
        let page_timings_json = "{
                                     \"onContentLoad\": -1,
                                     \"onLoad\": -1
                                 }";
        let page_timings_from_str: PageTimings = serde_json::from_str(page_timings_json).unwrap();
        assert_eq!(page_timings_from_str, page_timings );
    }

    #[test]
    fn test_entry() {
        let entry = Entry::new(
            Some("page_0".to_string()),
            "2009-04-16T12:07:23.596Z".to_string(),
            Request::new(
                "GET".to_string(),
                "http://www.example.com/path/?param=value".to_string(),
                "HTTP/1.1".to_string(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                None,
                None,
                None,
                None
            ),
            Response::new(
                200,
                "OK".to_string(),
                "HTTP/1.1".to_string(),
                Vec::new(),
                Vec::new(),
                Content::new(
                    100,
                    None,
                    "text/html; charset=utf8".to_string(),
                    None,
                    None,
                    None
                ),
                "".to_string(),
                None,
                None,
                None
            ),
            Cache::new(
                Absent,
                Absent,
                None
            ),
            Timing::new(
                TimedContent(1),
                TimedContent(2),
                TimedContent(3),
                4,
                5,
                6,
                TimedContent(7),
                None
            ),
            Some("10.0.0.1".to_string()),
            Some("52492".to_string()),
            Some("Comment".to_string())
        );
        let entry_json = "{
                              \"pageref\": \"page_0\",
                              \"startedDateTime\": \"2009-04-16T12:07:23.596Z\",
                              \"request\": {
                                  \"method\": \"GET\",
                                  \"url\": \"http://www.example.com/path/?param=value\",
                                  \"httpVersion\": \"HTTP/1.1\",
                                  \"cookies\": [],
                                  \"headers\": [],
                                  \"queryString\": []
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
                                  \"redirectURL\": \"\"
                              },
                              \"cache\": {
                                    \"beforeRequest\": null,
                                    \"afterRequest\": null
                              },
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
                              \"serverIpAddress\": \"10.0.0.1\",
                              \"connection\": \"52492\",
                              \"comment\": \"Comment\"
                          }";
        let entry_from_str: Entry = serde_json::from_str(entry_json).unwrap();
        assert_eq!(entry_from_str, entry );
    }

    #[test]
    fn test_entry_no_optional() {
        let entry = Entry::new(
            None,
            "2009-04-16T12:07:23.596Z".to_string(),
            Request::new(
                "GET".to_string(),
                "http://www.example.com/path/?param=value".to_string(),
                "HTTP/1.1".to_string(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                None,
                None,
                None,
                None
            ),
            Response::new(
                200,
                "OK".to_string(),
                "HTTP/1.1".to_string(),
                Vec::new(),
                Vec::new(),
                Content::new(
                    100,
                    None,
                    "text/html; charset=utf8".to_string(),
                    None,
                    None,
                    None
                ),
                "".to_string(),
                None,
                None,
                None
            ),
            Cache::new(
                Absent,
                Absent,
                None
            ),
            Timing::new(
                NotApplicable,
                NotApplicable,
                NotApplicable,
                4,
                5,
                6,
                NotApplicable,
                None
            ),
            None,
            None,
            None
        );
        let entry_json = "{
                              \"startedDateTime\": \"2009-04-16T12:07:23.596Z\",
                              \"request\": {
                                  \"method\": \"GET\",
                                  \"url\": \"http://www.example.com/path/?param=value\",
                                  \"httpVersion\": \"HTTP/1.1\",
                                  \"cookies\": [],
                                  \"headers\": [],
                                  \"queryString\": []
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
                                  \"redirectURL\": \"\"
                              },
                              \"cache\": {
                                    \"beforeRequest\": null,
                                    \"afterRequest\": null
                              },
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
        let entry_from_str: Entry = serde_json::from_str(entry_json).unwrap();
        assert_eq!(entry_from_str, entry );
        
    }

    #[test]
    fn test_request() {
        let request = Request::new(
            "GET".to_string(),
            "http://www.example.com/path/?param=value".to_string(),
            "HTTP/1.1".to_string(),
            vec![ Cookie::new(
                "TestCookie".to_string(),
                "Cookie Value".to_string(),
                None,
                None,
                None,
                None,
                None,
                None
            )],
            vec![Header::new(
                "Accept-Encoding".to_string(),
                "gzip,deflate".to_string(),
                None
            )],
            vec![QueryStringPair::new(
                "param1".to_string(),
                "value1".to_string(),
                None
            )],
            Some(PostData::new(
                "multipart/form-data".to_string(),
                Vec::new(),
                "plain posted data".to_string(),
                None
            )),
            Some(150),
            Some(0),
            Some("Comment".to_string())
        );
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
        let request_from_str: Request = serde_json::from_str(request_json).unwrap();
        assert_eq!(request_from_str, request );
    }

    #[test]
    fn test_request_no_optional() {
        let request = Request::new(
            "GET".to_string(),
            "http://www.example.com/path/?param=value".to_string(),
            "HTTP/1.1".to_string(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
            None,
            None,
            None
        );
        let request_json = "{
                                \"method\": \"GET\",
                                \"url\": \"http://www.example.com/path/?param=value\",
                                \"httpVersion\": \"HTTP/1.1\",
                                \"cookies\": [],
                                \"headers\": [],
                                \"queryString\": []
                            }";        
        let request_from_str: Request = serde_json::from_str(request_json).unwrap();
        assert_eq!(request_from_str, request );
    }

    #[test]
    fn test_response() {
        let response = Response::new(
            200, 
            "OK".to_string(), 
            "HTTP/1.1".to_string(), 
            Vec::new(), 
            Vec::new(), 
            Content::new(100, None, "text/html; charset=utf8".to_string(), None, None, None), 
            "".to_string(), 
            Some(160),
            Some(850),
            Some("".to_string())
        );
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
        let response_from_str: Response = serde_json::from_str(response_json).unwrap();
        assert_eq!(response_from_str, response );
    }

    #[test]
    fn test_response_no_optional() {
        let response = Response::new(
            200, 
            "OK".to_string(), 
            "HTTP/1.1".to_string(), 
            Vec::new(), 
            Vec::new(), 
            Content::new(100, None, "text/html; charset=utf8".to_string(), None, None, None), 
            "".to_string(), 
            None,
            None,
            None
        );

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
                                \"redirectURL\": \"\"
                            }";
        let response_from_str: Response = serde_json::from_str(response_json).unwrap();
        assert_eq!(response_from_str, response );
    }

    #[test]
    fn test_cookie() {
        let cookie = Cookie::new(
            "TestCookie".to_string(),
            "Cookie Value".to_string(), 
            Some("/".to_string()), 
            Some("www.janodvarko.cz".to_string()), 
            Some("2009-07-24T19:20:30.123+02:00".to_string()), 
            Some(false), 
            Some(false), 
            Some("".to_string())
        );
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
        let cookie_from_str: Cookie = serde_json::from_str(cookie_json).unwrap();
        assert_eq!(cookie_from_str, cookie );
    }

    #[test]
    fn test_cookie_no_optional() {
        let cookie = Cookie::new(
            "TestCookie".to_string(),
            "Cookie Value".to_string(),
            None,
            None,
            None,
            None,
            None,
            None
        );
        let cookie_json = "{
                               \"name\": \"TestCookie\",
                               \"value\": \"Cookie Value\"
                           }";
        let cookie_from_str: Cookie = serde_json::from_str(cookie_json).unwrap();
        assert_eq!(cookie_from_str, cookie );
    }

    #[test]
    fn test_header() {
        let header = Header::new(
            "Accept-Encoding".to_string(),
            "gzip,deflate".to_string(),
            Some("Comment".to_string())
        );
        let header_json = "{
                               \"name\": \"Accept-Encoding\",
                               \"value\": \"gzip,deflate\",
                               \"comment\": \"Comment\"
                           }";
        let header_from_str: Header = serde_json::from_str(header_json).unwrap();
        assert_eq!(header_from_str, header );
    }

    #[test]
    fn test_header_no_optional() {
        let header = Header::new(
            "Accept-Encoding".to_string(),
            "gzip,deflate".to_string(),
            None
        );
        let header_json = "{
                               \"name\": \"Accept-Encoding\",
                               \"value\": \"gzip,deflate\"
                           }";
        let header_from_str: Header = serde_json::from_str(header_json).unwrap();
        assert_eq!(header_from_str, header );
    }

    #[test]
    fn test_query_string_pair() {
        let query_string_pair = QueryStringPair::new(
            "param1".to_string(),
            "value1".to_string(),
            Some("Comment".to_string())
        );
        let query_string_pair_json = "{
                                          \"name\": \"param1\",
                                          \"value\": \"value1\",
                                          \"comment\": \"Comment\"
                                      }";
        let query_string_pair_from_str: QueryStringPair = serde_json::from_str(query_string_pair_json).unwrap();
        assert_eq!(query_string_pair_from_str, query_string_pair );
    }

    #[test]
    fn test_query_string_pair_no_optional() {
        let query_string_pair = QueryStringPair::new(
            "param1".to_string(),
            "value1".to_string(),
            None
        );
        let query_string_pair_json = "{
                                          \"name\": \"param1\",
                                          \"value\": \"value1\"
                                      }";
        let query_string_pair_from_str: QueryStringPair = serde_json::from_str(query_string_pair_json).unwrap();
        assert_eq!(query_string_pair_from_str, query_string_pair );
    }

    #[test]
    fn test_post_data() {
        let post_data = PostData::new(
            "multipart/form-data".to_string(),
            vec![Param::new( "paramName".to_string(), None, None, None, None)],
            "plain posted data".to_string(),
            Some("Comment".to_string())
        );
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
        let post_data_from_str: PostData = serde_json::from_str(post_data_json).unwrap();
        assert_eq!(post_data_from_str, post_data );
    }

    #[test]
    fn test_post_data_no_optional() {
        let post_data = PostData::new(
            "multipart/form-data".to_string(),
            Vec::new(),
            "plain posted data".to_string(),
            None
        );
        let post_data_json = "{
                                  \"mimeType\": \"multipart/form-data\",
                                  \"params\": [],
                                  \"text\": \"plain posted data\"
                              }";
        let post_data_from_str: PostData = serde_json::from_str(post_data_json).unwrap();
        assert_eq!(post_data_from_str, post_data );
    }

    #[test]
    fn test_param() {
        let param = Param::new(
            "paramName".to_string(),
            Some("paramValue".to_string()),
            Some("example.pdf".to_string()),
            Some("application/pdf".to_string()),
            Some("Comment".to_string())
        );
        let param_json = "{
                              \"name\": \"paramName\",
                              \"value\": \"paramValue\",
                              \"fileName\": \"example.pdf\",
                              \"contentType\": \"application/pdf\",
                              \"comment\": \"Comment\"
                          }";
        let param_from_str: Param = serde_json::from_str(param_json).unwrap();
        assert_eq!(param_from_str, param );
    }

    #[test]
    fn test_param_no_optional() {
        let param = Param::new(
            "paramName".to_string(),
            None,
            None,
            None,
            None
        );
        let param_json = "{
                              \"name\": \"paramName\"
                          }";
        let param_from_str: Param = serde_json::from_str(param_json).unwrap();
        assert_eq!(param_from_str, param );
    }

    #[test]
    fn test_content() {
        let content = Content::new(
            100, Some(0),
            "text/html; charset=utf8".to_string(),
            Some("\n".to_string()),
            Some("base64".to_string()),
            Some("Comment".to_string())
        );
        let content_json = "{
                                \"size\": 100,
                                \"compression\": 0,
                                \"mimeType\": \"text/html; charset=utf8\",
                                \"text\": \"\\n\",
                                \"encoding\": \"base64\",
                                \"comment\": \"Comment\"
                            }";
        let content_from_str: Content = serde_json::from_str(content_json).unwrap();
        assert_eq!(content_from_str, content );
    }

    #[test]
    fn test_content_no_optional() {
        let content = Content::new(
            100, None,
            "text/html; charset=utf8".to_string(),
            None,
            None,
            None
        );
        let content_json = "{
                                \"size\": 100,
                                \"mimeType\": \"text/html; charset=utf8\"
                            }";
        let content_from_str: Content = serde_json::from_str(content_json).unwrap();
        assert_eq!(content_from_str, content );
    }

    #[test]
    fn test_cache() {
        let cache = Cache::new(
            Present(CacheEntry::new(
                None, 
                "2000-01-01T00:00:00.000Z".to_string(), 
                "123456789".to_string(),
                42,
                None
            )),
            Present(CacheEntry::new(
                None, 
                "2000-02-01T00:00:00.000Z".to_string(), 
                "987654321".to_string(), 
                24, 
                None
            )),
            Some("Comment".to_string())
        );
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
        let cache_from_str: Cache = serde_json::from_str(cache_json).unwrap();
        assert_eq!(cache_from_str, cache );
    }

    #[test]
    fn test_cache_absent_entries() {
        let cache = Cache::new(
            Absent,
            Absent,
            None
        );
        let cache_json = "{
                              \"beforeRequest\": null,
                              \"afterRequest\": null
                          }";
        let cache_from_str: Cache = serde_json::from_str(cache_json).unwrap();
        assert_eq!(cache_from_str, cache );
    }

    #[test]
    fn test_cache_unknown_entries() {
        let cache = Cache::new(
            Unknown,
            Unknown,
            None
        );
        let cache_json = "{}";
        let cache_from_str: Cache = serde_json::from_str(cache_json).unwrap();
        assert_eq!(cache_from_str, cache );
    }


    #[test]
    fn test_cache_entry() {
        let cache_entry = CacheEntry::new(
            Some("2000-02-01T00:00:00.000Z".to_string()), 
            "2000-01-01T00:00:00.000Z".to_string(), 
            "123456789".to_string(),
            42,
            Some("Comment".to_string())
        );
        let cache_entry_json = "{
                                    \"expires\": \"2000-02-01T00:00:00.000Z\",
                                    \"lastAccess\": \"2000-01-01T00:00:00.000Z\",
                                    \"eTag\": \"123456789\",
                                    \"hitCount\": 42,
                                    \"comment\": \"Comment\"
                                }";
        let cache_entry_from_str: CacheEntry = serde_json::from_str(cache_entry_json).unwrap();
        assert_eq!(cache_entry_from_str, cache_entry );
    }

    #[test]
    fn test_cache_entry_no_optional() {
        let cache_entry = CacheEntry::new(
            None, 
            "2000-01-01T00:00:00.000Z".to_string(), 
            "123456789".to_string(),
            42,
            None
        );
        let cache_entry_json = "{
                                    \"lastAccess\": \"2000-01-01T00:00:00.000Z\",
                                    \"eTag\": \"123456789\",
                                    \"hitCount\": 42
                                }";
        let cache_entry_from_str: CacheEntry = serde_json::from_str(cache_entry_json).unwrap();
        assert_eq!(cache_entry_from_str, cache_entry );
    }
    #[test]
    fn test_timing() {
        
        let timing = Timing::new(
            TimedContent(1), 
            TimedContent(2), 
            TimedContent(3), 
            4,
            5,
            6,
            TimedContent(7), 
            Some("Comment".to_string())
        );
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
        let timing_from_str: Timing = serde_json::from_str(timing_json).unwrap();
        assert_eq!(timing_from_str, timing );
    }

    #[test]
    fn test_timing_no_optional() {
        let timing = Timing::new(
            NotApplicable, 
            NotApplicable, 
            NotApplicable, 
            4, 
            5, 
            6, 
            NotApplicable, 
            None
        );
        let timing_json = "{
                                \"blocked\": -1,
                                \"dns\": -1,
                                \"connect\": -1,
                                \"send\": 4,
                                \"wait\": 5,
                                \"receive\": 6,
                                \"ssl\": -1
                           }";
        let timing_from_str: Timing = serde_json::from_str(timing_json).unwrap();
        assert_eq!(timing_from_str, timing );
    }
}
