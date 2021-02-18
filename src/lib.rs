pub fn raindrops(n: u32) -> String {
    let mut rainSound = String::new();

    if n % 3 == 0 {
      rainSound += "Pling";
    }

    if n % 5 == 0 {
      rainSound += "Plang";
    }

    if n % 7 == 0 {
      rainSound += "Plong";
    }
    
    if rainSound.len() == 0 {
	rainSound = n.toString();
	}

    sound

}

