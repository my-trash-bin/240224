use std::io;

use test00::SinglyLinkedList;

fn main() {
    let mut sll = SinglyLinkedList::new();

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim().to_string();
        if line == "exit" {
            break;
        }
        if let Ok(n) = line.parse() {
            println!("{:#?}", sll.nth(n));
        } else {
            sll.push(line);
        }
    }
}
