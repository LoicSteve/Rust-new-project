/* A Marco Polo Game

If the name Marco is given, the program will respond with Polo.
If the name Marco is not given, the program will respond with What is your name?
*/

pub fn marco(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        "What is your name?".to_string()
    }
}
