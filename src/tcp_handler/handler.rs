pub struct RequestHandler;

impl RequestHandler {
    pub fn handle_request(request: String) -> String {
        println!("Handling request: {}", request);

        let input = if let Some(index) = request.find("command?input=") {
            let start = index + "command?input=".len();

            if let Some(end) = request[start..].find(" HTTP/1.1") {
                &request[start..(start + end)]
            } else {
                ""
            }
        } else {
            ""
        };

        let response_text = match input {
            "flag" => "Congradulations! flag: I_LOVE_COOKIES".to_string(),
            _ => "Try again..".to_string(),
        };
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            response_text.len(),
            response_text
        );
        response
    }
}
