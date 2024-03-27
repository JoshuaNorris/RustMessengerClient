use std::{io, thread};
use std::time::Duration;
use std::net::{TcpStream};
use std::io::{Read, Write, stdin, ErrorKind, Error};
use std::str::from_utf8;
use MessengerAppClient::{messages, CONN, ADDRESS};
use std::collections::LinkedList;

// The IP_ADDRESS needs to be constantly updated depending on the machine this program is running on.


fn main() {
    messages::init_messages();

    let username = get_username();
    let username2 = username.clone();

    thread::spawn(move|| {
        get_new_messages(&username2);
    });

    loop {
        let mut message = get_message(&username);
        if message.contains("exit") {
            break
        } else {
            {
                let address = ADDRESS.lock().expect("Could not lock the address.").to_string();
                let mut stream = TcpStream::connect(address).expect("Could not create stream.");
                stream.write(&message.as_bytes()).unwrap();
                get_response(stream, message);
            }
        }
    }
}

fn get_response(mut stream: TcpStream, message: String) -> io::Result<()> {
    let mut split_message = message.split_whitespace();

    match split_message.next().expect("Could not get the split_message.") {
        "POST" => {
            post_command(stream, message);
        }
        "GET" => {
            get_command(stream, message);
        }
        _ => {
        }
    }
    Ok(())
}

fn post_command(mut stream: TcpStream, message: String) {
    let mut data = [0 as u8; 512]; // using 6 byte buffer
    let address = ADDRESS.lock().expect("Could not lock the address.").to_string();
    stream.read(&mut data).expect("could not read the data.");
    let response = String::from_utf8_lossy(&data).into_owned();
    println!("{}", response);

}

fn get_command(mut stream: TcpStream, message: String) {
    let mut data = [0 as u8; 512]; // using 6 byte buffer
    stream.read(&mut data).expect("Could not read the data.");
    let mut response = String::from_utf8_lossy(&data).into_owned();
    response = String::from(response.trim_matches(char::from(0)));
    if response != "null" {
        // there is an unwrap of a none value in parse_conversation
        let mut messages: LinkedList<messages::Messages> = messages::parse_conversation(response);
        for message in messages {
            message.insert();
        }
    }
}

fn get_username() -> String {
    println!("> What would you like your username to be?");
    let mut message = String::new();
    stdin().read_line(&mut message);
    String::from(message.trim_right_matches('\n'))
}

fn get_message(username: &String) -> String {
    println!("> Would you like to view a conversation, send a message, or exit?");
    println!("> view, send, or exit.");
    let mut message = String::new();
    stdin().read_line(&mut message);

    if message.contains("view") {
        view_message(&username)
    } else if message.contains("send") {
        send_message(&username)
    } else if message.contains("exit"){
        exit_message()
    } else {
        String::new()
    }
}

fn exit_message() -> String {
    String::from("exit")
}

fn send_message(username: &String) -> String {
    println!("> Who would you like to send a message to?");
    let mut to_user = String::new();
    stdin().read_line(&mut to_user);

    println!("> What is the message?");
    let mut message = String::new();
    stdin().read_line(&mut message);


    let username = String::from(username.trim_right_matches('\n'));
    let to_user = String::from(to_user.trim_right_matches('\n'));
    let contents = String::from(message.trim_right_matches('\n'));

    let message = messages::Messages::new(username.clone(), to_user.clone(), message.clone());

    //message.insert();

    format!("POST {} {} {}", username.trim_right_matches('\n'), to_user.trim_right_matches('\n'), contents.trim_right_matches('\n'))
}

fn view_message(username: &String) -> String {
    println!("> Which conversation would you like the view?");
    let mut to_user = String::new();
    stdin().read_line(&mut to_user);
    let to_user = String::from(to_user.trim_right_matches('\n'));
    let mes = messages::get_conversation(username, to_user).expect("Could get messages from Client's Database.");
    println!("{}", &mes);
    mes
}

fn get_new_messages(username: &String) {
    loop {
        thread::sleep(Duration::new(1, 0));
        //let conn = CONN.lock().expect("Problem getting CONN");
            let mut message = String::new();
        {
            let address = ADDRESS.lock().expect("Could not lock the address.").to_string();
            let mut stream = TcpStream::connect(address).expect("Could not connect stream.");
            message = format!("GET {}", username);

            stream.write(&message.as_bytes()).expect("Could not write to the stream.");
            get_response(stream, message);
        }
    }

}