use modules::socket_handler::Socket;


mod tests;
mod modules;



const SERVER_ADDRESS: &str = "localhost:5000";

fn main() {
    let listener = Socket::make_listener(SERVER_ADDRESS);

    

    drop(listener);
}
