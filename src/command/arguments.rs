use regex::Regex;
use reqwest::Response;
use reqwest::header::{HeaderMap, GetAll, HeaderValue};

pub fn check_arg_type(argument: Box<dyn LakituArgument>, value: String) -> bool {
    lazy_static! {
        static ref urlRegex: Regex = Regex::new(r"(http(s)?:\/\/.)?(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)").unwrap();
        static ref imageMimeRegex: Regex = Regex::new(r"image\/[jpeg|pjpeg|jp2|jpx|gif|png|bmp|svg+xml|x\-icon|vnd.microsoft.icon|vnd.djvu]").unwrap();
    }

    match argument.get_type() {
        ArgumentType::String => {
            true
        }
        ArgumentType::Integer => {
            match value.parse::<i32>() {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        ArgumentType::Float => {
            match value.parse::<f64>() {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        ArgumentType::Image => {
            let matches_url: bool = urlRegex.is_match(value.to_string());

            // Checks the MIME type of the URL provided
            if matches_url {
                let resp: Response = reqwest::get(value)?;
                let headers: &HeaderMap = resp.headers();

                if !headers.contains_key("Content-Type") { false }

                let content_types: GetAll<HeaderValue> = headers.get_all("Content-Type");
                let mut any_match: bool = false;

                for content_type in content_types {
                    let type_str: &str = content_type.to_str()?;
                    let image_match: bool = imageMimeRegex.is_match(type_str);

                    if image_match {
                        any_match = true;
                        break;
                    }
                }

                any_match
            }

            false
        }
        ArgumentType::User => {
            // Todo
        }
    }
}

pub enum ArgumentType {
    String,
    Integer,
    Float,
    Image,
    User,
}

pub trait LakituArgument {
    fn get_name(&self) -> String;
    fn get_desc(&self) -> String;

    fn get_type(&self) -> ArgumentType;

    fn is_required(&self) -> bool;
}