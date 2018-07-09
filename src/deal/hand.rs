use super::Card;

#[derive(Debug, Clone)]
pub struct Hand {
    id: u32,
    values: Vec<Card>,
}

impl Hand {
    pub fn new(id: u32, values: &Vec<Card>) -> Hand {
        Hand {
            id,
            values: values.to_vec(),
        }
    }

    pub fn get(&self) -> Vec<Card> {
        self.values.to_vec()
    }

    pub fn add(&self, new_card: Card) {
        let mut values = self.values;
        values.push(new_card);
        self.values = values.to_vec();
    }

    pub fn total(&self) -> u32 {
        let mut total = 0;
        // TODO handle ace being 1 or 11
        for c in self.values {
            total += c.value;
        }

        total
    }
}
