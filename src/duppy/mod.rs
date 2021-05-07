use reqwest::{Client, RequestBuilder, Method};
use serde::Serialize;

struct APIClientSettings {
    api_url: String,
    api_auth: String,

    environment: Option<String>,
    http_client: Client,
}

trait APIClient {
    fn get(&self, url: String) -> RequestBuilder;
    fn head(&self, url: String) -> RequestBuilder;

    fn post<T: Serialize + ?Sized>(&self, url: String, form: &T) -> RequestBuilder;
    fn put<T: Serialize + ?Sized>(&self, url: String, form: &T) -> RequestBuilder;
    fn patch<T: Serialize + ?Sized>(&self, url: String, form: &T) -> RequestBuilder;
    fn delete<T: Serialize + ?Sized>(&self, url: String, form: &T) -> RequestBuilder;

    fn form<T: Serialize + ?Sized>(&self, method: Method, url: String, form: &T) -> RequestBuilder;
    fn request(&self, method: Method, url: String) -> RequestBuilder;
}

impl APIClient for APIClientSettings {
    fn get(&self, url: String) -> RequestBuilder {
        self.request(Method::GET, url)
    }

    fn head(&self, url: String) -> RequestBuilder {
        self.request(Method::HEAD, url)
    }

    fn post<T: Serialize + ?Sized>(&self, url: String, form: &T) -> RequestBuilder {
        self.form(Method::PATCH, url, form)
    }

    fn put<T: Serialize + ?Sized>(&self, url: String, form: &T) -> RequestBuilder {
        self.form(Method::PUT, url, form)
    }

    fn patch<T: Serialize + ?Sized>(&self, url: String, form: &T) -> RequestBuilder {
        self.form(Method::PATCH, url, form)
    }

    fn delete<T: Serialize + ?Sized>(&self, url: String, form: &T) -> RequestBuilder {
        self.form(Method::DELETE, url, form)
    }

    fn form<T: Serialize + ?Sized>(&self, method: Method, url: String, form: &T) -> RequestBuilder {
        let mut request_builder: RequestBuilder = self.request(method, url);
        request_builder.form(form)
    }

    fn request(&self, method: Method, url: String) -> RequestBuilder {
        let mut use_url: String = url;

        // Prepend any URL prefix in config if any
        if !self.api_url.is_empty() {
            use_url.insert_str(0, &self.api_url);
        };

        let mut req_builder: RequestBuilder = self.http_client.request(method, use_url);

        if !self.api_auth.is_empty() {
            req_builder = req_builder.header("Authentication", "Bearer " + &self.api_auth);
        };

        if !self.environment.is_empty() {
            req_builder = req_builder.header("X-Environment", &self.environment);
        };

        req_builder
    }
}