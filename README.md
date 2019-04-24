# Diamond

## problem
in this problem, we are given a char C, and we have to make a diamond shape, it prints a diamond starting with 'A', with the supplied letter at the widest point.

Requirements
- The first row contains one 'A'.
- The last row contains one 'A'.
- All rows, except the first and last, have exactly two identical letters.
- All rows have as many trailing spaces as leading spaces. (This might be 0).
- The diamond is horizontally symmetric.
- The diamond is vertically symmetric.
- The diamond has a square shape (width equals height).
- The letters form a diamond shape.
- The top half has the letters in ascending order.
- The bottom half has the letters in descending order.
- The four corners (containing the spaces) are triangles.

Example
diamonds for letter C
```
··A··
·B·B·
C···C
·B·B·
··A··
```


## solution 
first, we make result variable as a vec : Vec<String> 

```rust
pub fn get_diamond(c: char) -> Vec<String> {
    let mut result:Vec<String> = Vec::new();
    let alph:Vec<char> = String::from_utf8((b'A'..=b'Z').collect()).unwrap().chars().collect();
    for x in 0..(c as usize)-64{
        let mut temp = "".to_string();
        for y in 0..(((c as usize)-64)*2)-1{
            if y == ((c as usize)-65)+x || y == ((c as usize)-65)-x{
                temp = format!("{}{}",temp,alph[x]);
            }
            else{
                temp = format!("{}{}",temp," ");
            }
            
        }
        if x == (c as usize)-65{
            result.insert(result.len()/2,temp);
        }
        else{
            result.insert(result.len()/2,temp.clone());
            result.insert(result.len()/2,temp);
        }
        //println!("{} {}",x ,(c as usize)-65);
    }
    result
}
```
saya check semua kolom apakah kolom itu harus diisi dengan char atau tidak 
```rust 
        for y in 0..(((c as usize)-64)*2)-1{
            if y == ((c as usize)-65)+x || y == ((c as usize)-65)-x{
                temp = format!("{}{}",temp,alph[x]);
            }
            else{
                temp = format!("{}{}",temp," ");
            }
```
dan hasil nya akan menjadi sebuah string


lalu check, jika iterasi ini adalah yang terakhir, maka result hanya akan ditambah 1 kali dari tengah
jika tidak,maka result ditambahkan 2 kali dari tengah

```rust
for x in 0..(c as usize)-64{
        let mut temp = "".to_string();
        .....
        if x == (c as usize)-65{
            result.insert(result.len()/2,temp);
        }
        else{
            result.insert(result.len()/2,temp.clone()); // inser(index element position , element)
            result.insert(result.len()/2,temp);
        }
        //println!("{} {}",x ,(c as usize)-65);
    }
```



##### Muhammad Luthfi A
##### Nickname : LLuthfiY
- clock
- pythagorean triplet
- diamond
- pascal's triangle 
- triagle
- scrabble
 
