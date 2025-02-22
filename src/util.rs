use rand::Rng;

const AVAILABLE_CHARS: [char; 57] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q',
    'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w',
    'x', 'y', 'z', '2', '3', '4', '5', '6', '7', '8', '9', '-'
];

pub fn gen_random_id(length: i32) -> String {
    let mut rng = rand::rng();
    let random_id: String = (0..length)
        .map(|_| rng.random_range(0..AVAILABLE_CHARS.len()) as usize)
        .map(|index| AVAILABLE_CHARS[index])
        .collect();
    random_id
}
