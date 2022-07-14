fn main() {
    ///
    ///
    ///

    /// Stores a polynomial in coefficient form.

    struct Character {
        name: String,
        polynomial: Vec<i32>,
        random_input: i32,
    }

    impl Character {
        fn evaluate_polyn(&self) -> i32 {
            let mut output_vec: Vec<i32> = Vec::new();
            let mut result = 0;
            for i in self.polynomial.clone() {
                let index = self.polynomial.iter().position(|&j| j == i).unwrap();
                output_vec.push(i * self.random_input.pow(index as u32));
            }
            for i in output_vec {
                result += i;
            }
            println!(
                "{}'s evaluated polynomial p({}) = {}",
                self.name, self.random_input, result
            );
            result
        }

        fn compare(&self, other_character: Character) -> bool {
            if self.evaluate_polyn() == other_character.evaluate_polyn() {
                println!("a_n = b_n with a high probability");
                return true;
            } else {
                println!("a_n is not equal to b_n");
                return false;
            }
        }
    }

    let alice = Character {
        name: "Alice".to_string(),
        polynomial: vec![10, 20, 30, 40, 80, 100],
        random_input: 4,
    };

    let bob = Character {
        name: "Bob".to_string(),
        polynomial: vec![10, 20, 30, 40, 80, 100],
        random_input: 4,
    };

    assert_eq!(bob.compare(alice), true);
}
