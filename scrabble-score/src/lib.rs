/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut scr = 0;
    for x in word.to_lowercase().chars(){
        scr += match x {
            'a'|'i'|'u'|'e'|'o'|'l'|'n'|'r'|'s'|'t' =>  1,
            'd'|'g' => 2,
            'b'|'c'|'m'|'p' => 3,
            'f'|'h'|'v'|'w'|'y' => 4,
            'k' => 5,
            'j'|'x' => 8,
            'q'|'z' => 10,
            _ => 0,
        }
    }
    scr
}