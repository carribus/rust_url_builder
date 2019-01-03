//! # URLBuilder
//! 
//! An easy-to-use crate to construct URLs for the Rust Programming language
//! 
//! You can use this to build up context for a url over the course of execution and then
//! call the `.build()` method to generate the final url.
//! 
//! The mutating functions allow you to chain them to each other.
//! 
//! ## Example
//! 
//! The following code will create a url similar to `http://localhost:8000?first=1&second=2&third=3`
//! The order of the query parameters is indeterminate as the parameters are internally stored in 
//! `std::collections::HashMap`.
//! 
//! ```
//! let mut ub = URLBuilder::new();
//! 
//! ub.set_protocol("http")
//!     .set_host("localhost")
//!     .set_port(8000)
//!     .add_param("first", "1")
//!     .add_param("second", "2")
//!     .add_param("third", "3");
//! 
//! println!("{}", ub.build()); 
//! ```

use std::collections::HashMap;

pub struct URLBuilder {
    protocol: String,
    host: String,
    port: i16,
    params: HashMap<String, String>,
}

impl URLBuilder {
    /// Use this method to create a new URLBuilder instance
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut ub = URLBuilder::new();
    /// ```
    pub fn new() -> URLBuilder {
        URLBuilder {
            protocol: String::new(),
            host: String::new(),
            port: 0,
            params: HashMap::new(),
        }
    }

    /// Use this method to generate a URL string
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut ub = URLBuilder::new();
    /// let url = ub.set_protocol("http")
    ///             .set_host("127.0.0.1")
    ///             .set_port(8000)
    ///             .build();
    /// ```
    pub fn build(&self) -> String {
        let base = format!("{}://{}:{}", self.protocol, self.host, self.port);
        let mut query = String::new();
        if self.params.len() > 0 {
            query.push('?');
            for (key, value) in self.params.iter() {
                query.push_str(format!("{}={}&", key, value).as_str());
            }
        }
        format!("{}{}", base, query)
    }

    /// Adds a query parameter that will be added to the generated URL
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut ub = URLBuilder::new();
    /// let url = ub.set_protocol("http")
    ///             .set_host("127.0.0.1")
    ///             .set_port(8000)
    ///             .add_param("parameter", "some_value")
    /// ```
    pub fn add_param(&mut self, param: &str, value: &str) -> &mut Self{
        self.params.insert(param.to_string(), value.to_string());
        self
    }

    /// Sets the protocol to use in the URL
    pub fn set_protocol(&mut self, prot: &str) -> &mut Self {
        self.protocol = String::from(prot);
        self
    }

    /// Sets the host for the URL
    pub fn set_host(&mut self, host: &str) -> &mut Self {
        self.host = String::from(host);
        self
    }

    /// sets the port for the URL
    pub fn set_port(&mut self, port: i16) -> &mut Self {
        self.port = port;
        self
    }

    /// Returns the port in the URLBuilder instance
    pub fn port(&self) -> i16 {
        self.port
    }

    /// Returns the host in the URLBuilder instance
    pub fn host(&self) -> &str {
        &self.host
    }

    /// returns the protocol in the URLBuilder instance
    pub fn protocol(&self) -> &str {
        &self.protocol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_host() {
        let mut ub = URLBuilder::new();
        ub.set_host("localhost");
        assert_eq!("localhost", ub.host());
    }

    #[test]
    fn test_set_protocol() {
        let mut ub = URLBuilder::new();
        ub.set_protocol("https");
        assert_eq!("https", ub.protocol());
    }

    #[test]
    fn test_set_port() {
        let mut ub = URLBuilder::new();
        ub.set_port(8000);
        assert_eq!(8000, ub.port());
    }

    #[test]
    fn create_google_url() {
        let mut ub = URLBuilder::new();
        ub.set_protocol("http")
            .set_host("www.google.com")
            .set_port(80); 
        let url = ub.build();
        assert_eq!("http://www.google.com:80", url);
    }

    #[test]
    fn create_url_with_params() {
        let mut ub = URLBuilder::new();
        ub.set_protocol("http")
            .set_host("localhost")
            .set_port(8000)
            .add_param("first", "1")
            .add_param("second", "2")
            .add_param("third", "3");

        let url = ub.build();
        assert!(url.contains("first=1"));
        assert!(url.contains("second=2"));
        assert!(url.contains("third=3"));
    }
}
