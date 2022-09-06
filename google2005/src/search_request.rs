use urlencoding::decode;

pub struct SearchRequest<'a> {
    pub params: &'a str,
    pub search_string: String,
    pub start: u16,
}

impl<'a> SearchRequest<'a> {
    pub fn new(params_string: &'a str) -> Self {
        SearchRequest {
            params: params_string,
            search_string: Self::search_string(params_string),
            start: Self::start(params_string),
        }
    }

    pub fn search_string(params_string: &'a str) -> String {
        let q = params_string.split("&").collect::<Vec<&str>>()[0];

        let q = decode(q).unwrap();
        //replace + with space
        let q = q.replace("+", " ");

        q.to_string()
    }

    pub fn start(params_string: &'a str) -> u16 {
        let mut start = 0;

        for param in params_string.split("&") {
            if param.starts_with("start=") {
                start = param.split("=").collect::<Vec<&str>>()[1]
                    .parse::<u16>()
                    .unwrap();
            }
        }

        start
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_string() {
        let params = "cubs&start=0";

        assert_eq!(SearchRequest::search_string(params), "cubs".to_string());
    }

    #[test]
    fn test_search_string_decodes_params() {
        let params = "george+clooney&start=10";

        assert_eq!(SearchRequest::search_string(params), "george clooney");
    }

    #[test]
    fn test_start_defaults_to_zero() {
        let params = "cubs";

        assert_eq!(SearchRequest::start(params), 0);

        let params = "george+clooney&foo=bar";

        assert_eq!(SearchRequest::start(params), 0);
    }

    #[test]
    fn test_start_finds_start() {
        let params = "george+clooney&start=10";

        assert_eq!(SearchRequest::start(params), 10);
    }
}
