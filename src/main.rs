mod tcp_handler;

use tcp_handler::TcpHandler;

fn main() {
    let tcp_server = TcpHandler::build("127.0.0.1:8080");
    tcp_server.run();
}
