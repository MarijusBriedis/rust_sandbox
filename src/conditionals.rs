// Conditionals - used to check the condition of something and act on the result

pub fn run() {
    let age: u8 = 21;
    let _check_id: bool = true;
    let knows_person_of_age = true;

    // If/Else
    if age >= 21 && _check_id || knows_person_of_age {
        println!("Bartender: What would You like to drink?");
    } else if age < 21 && _check_id {
        println!("Bartender: Sorry, You have to leave.");
    } else {
        println!("Bartender: I'll need to see Your ID.");
    }

    // Shorthand if
     let is_of_age = if age >= 21 { true } else { false };
     println!("Is of age: {}", is_of_age);
}