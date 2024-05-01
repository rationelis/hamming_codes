use rand::Rng;

pub struct Encoder {
    pub bits_of_data: usize,
    pub bits_of_parity: usize,
}

impl Encoder {
    pub fn new(bits_of_data: usize, bits_of_parity: usize) -> Encoder {
        Encoder {
            bits_of_data,
            bits_of_parity,
        }
    }

    pub fn encode(&self, input: Vec<i32>) -> Result<Message, String> {
        let length = self.bits_of_data + self.bits_of_parity;

        let mut block = vec![0; length];

        let powers = Encoder::generate_powers(2, length as i32);
        
        let mut data_index = 0;
        for i in 1..length {
            if !powers.contains(&(i as i32)) {
                block[i] = input[data_index];
                data_index += 1;
            }
        }

        let parities = Encoder::get_parities(&block);

        for i in 0..self.bits_of_parity {
            block[powers[i] as usize] = parities[i];
        }

        let zero_parity = Encoder::get_parity_entire_block(&block);
        block[0] = zero_parity;

        Ok(Message::new(block))
    }

    pub fn validate_block(block: &Vec<i32>) -> bool {
        let parities = Encoder::get_parities(block);
        let zero_parity = Encoder::get_parity_entire_block(block);
        parities.iter().all(|&x| x == 0) && zero_parity == 0
    }

    pub fn get_parity_entire_block(block: &Vec<i32>) -> i32 {
        let mut parity = 0;
        for i in block {
            parity ^= i;
        }
        parity
    }

    pub fn get_parities(block: &Vec<i32>) -> Vec<i32> {
        let q1_and_3 = [1, 3];
        let q2_and_4 = [2, 3];

        let answer_q1 = Encoder::get_parity_of_columns(&block, q1_and_3.to_vec());
        let answer_q2 = Encoder::get_parity_of_columns(&block, q2_and_4.to_vec());
        let answer_q3 = Encoder::get_parity_of_rows(&block, q1_and_3.to_vec()); 
        let answer_q4 = Encoder::get_parity_of_rows(&block, q2_and_4.to_vec());

        vec![answer_q1, answer_q2, answer_q3, answer_q4]
    }

    pub fn get_parity_of_columns(block: &Vec<i32>, columns: Vec<usize>) -> i32 {
        let mut parity = 0;
        let height = (block.len() as f64).sqrt() as usize;
        for i in columns {
            for j in 0..height {
                parity ^= block[i + j * height];
            }
        } 
        parity
    }

    pub fn get_parity_of_rows(block: &Vec<i32>, rows: Vec<usize>) -> i32 {
        let mut parity = 0;
        let height = (block.len() as f64).sqrt() as usize;
        for i in rows {
            for j in 0..height {
                parity ^= block[i * height + j];
            }
        } 
        parity
    }

    pub fn generate_powers(base: i32, n: i32) -> Vec<i32> {
        let sqrt_n = (n as f64).sqrt() as i32;
        (0..sqrt_n).map(|i| base.pow(i as u32)).collect()
    }
}

pub struct Message {
    pub data: Vec<i32>,
}

impl Message {
    fn new(input: Vec<i32>) -> Message {
        Message {
            data: input,
        }
    } 
}

pub fn create_random_message(length: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen_range(0..=1)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_static_message() {
        let encoder = Encoder::new(12, 4);

        let input = vec![1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1];

        match encoder.encode(input) {
            Ok(message) => {
                let expected = vec![
                    1, 1, 0, 1,
                    0, 1, 0, 0,
                    1, 1, 0, 1,
                    1, 0, 1, 1
                ];

                assert_eq!(message.data, expected);
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    #[test]
    fn check_one_bit_error() {
        let encoder = Encoder::new(12, 4);

        let input = vec![1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1];

        match encoder.encode(input) {
            Ok(message) => {
                let mut altered = message.data.clone();
                altered[0] = 1 - altered[0];
                assert_eq!(Encoder::validate_block(&altered), false);   
            },
            Err(e) => {
                println!("{:?}", e);
            }
        } 
    }

    #[test]
    fn check_two_bit_error() {
        let encoder = Encoder::new(12, 4);

        let input = create_random_message(11);
    
        match encoder.encode(input.clone()) {
            Ok(message) => {
                let mut altered = message.data.clone();
                altered[5] = 1 - altered[5];
                altered[9] = 1 - altered[9];
                assert_eq!(Encoder::validate_block(&altered), false);

            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    #[test]
    fn check_random_message() {
        let encoder = Encoder::new(12, 4);

        let input = create_random_message(11);

        match encoder.encode(input) {
            Ok(message) => {
                assert_eq!(Encoder::validate_block(&message.data), true);
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    
    #[test]
    fn check_random_message_100k_times() {
        let encoder = Encoder::new(12, 4);

        for _ in 0..100000 {
            let input = create_random_message(11);

            match encoder.encode(input) {
                Ok(message) => {
                    assert_eq!(Encoder::validate_block(&message.data), true);
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    }
}

