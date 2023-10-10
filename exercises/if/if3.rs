// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.



pub fn animal_habitat(animal: &str) -> &'static str {
    let mut num=1;
    
    if animal == "crab" {
        num=1;
    } else if animal == "gopher" {
        num=2;
    } else if animal == "snake" {
        num=3;
    } else {
       num=4;
    };
    let identifier = num;

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

   return  habitat;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
