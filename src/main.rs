extern crate x11_clipboard;

use x11_clipboard::Clipboard;

fn main() {
    let clipboard = Clipboard::new().unwrap();

    println!("Atoms available:");
    dbg!(clipboard.getter.atoms.string);
    dbg!(clipboard.getter.atoms.utf8_string);
    dbg!(clipboard.getter.atoms.text_plain);

    println!("\nWaiting for selection...");

    loop {
        if let Ok(curr) = clipboard.load_wait(
            clipboard.getter.atoms.primary,
            clipboard.getter.atoms.utf8_string,
            clipboard.getter.atoms.property
        ) {
            let curr = String::from_utf8_lossy(&curr);
            let curr = curr
                .trim_matches('\u{0}')
                .trim();

            if !curr.is_empty() {
                println!("Contents of primary selection: {}", curr.to_owned());
                println!("\nWaiting for selection...");
            }
        }
    }
}
