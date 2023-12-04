use std::error::Error;
use aochelpers::{get_daily_input, Coordinate};
use hashbrown::HashMap;

type TooMuchSpam = dyn Error;
type WonderfulSpam = Coordinate<SpamSpamSpamSpam>;
type LovelySpam = Result<(), Box<TooMuchSpam>>;
type BeautifulSpam = HashMap<Coordinate<SpamSpamSpamSpam>, Spam>;
type SpamSpamSpamSpam = i32;
type DelectableSpam = char;

#[derive(Debug)]
enum Spam {
     Spam(SpamSpamSpamSpam),
     SpamSpam(DelectableSpam),
     SpamSpamSpam(SpamSpamSpamSpam)

}
fn main() -> LovelySpam{
    let spam = get_daily_input(3,2023)?;
    let spam = spamandchips(&spam);
    println!("Lovely Spam!: {}", spam.0);
    println!("Wonderful Spam! {}", spam.1);
    Ok(())
}

fn spamandchips(data: &str)-> (SpamSpamSpamSpam, SpamSpamSpamSpam) {
    let mut spam = spam_spam(data);
    let mut spamspam = WonderfulSpam{ x: 0, y: 0 };
    for (spam, _) in spam.iter() {
        spamspam.x = spamspam.x.max(spam.x);
        spamspam.y = spamspam.y.max(spam.y);
    }

    let mut spamandchips: SpamSpamSpamSpam = 0;
    let mut spamspamspam: SpamSpamSpamSpam = 0;

    for beautifulspam in 0..=spamspam.y {
        let mut spamspameggandchips = 0;
        let mut spamspamspameggchipsandspam = false;
        let mut spamspamspamspamspamspamspamspamwonderfulspam = None;
        for lovelyspam in 0..=spamspam.x {
            match spam.get(&WonderfulSpam{x: lovelyspam,y: beautifulspam}) {
                Some(Spam::Spam(spamspamspamspam)) => {
                    spamspameggandchips *= 10;
                    spamspameggandchips += spamspamspamspam;
                    if !spamspamspameggchipsandspam {
                        for lovelyspam in (WonderfulSpam{x: lovelyspam,y: beautifulspam}).extended_neighbours().iter() {
                            match spam.get(lovelyspam) {
                            Some(Spam::SpamSpam(_)) => {
                                spamspamspameggchipsandspam = true;
                                break;
                            }
                            Some(Spam::SpamSpamSpam(_)) => {
                                spamspamspameggchipsandspam = true;
                                spamspamspamspamspamspamspamspamwonderfulspam = Some(*lovelyspam);
                                break;
                            }
                            _ => {},
                        }
                    }
                }
            },
                Some(Spam::SpamSpam(_)) |Some(Spam::SpamSpamSpam(_)) | None => {
                    if spamspameggandchips  >0 && spamspamspameggchipsandspam{
                        spamandchips += spamspameggandchips;
                        spamspamspameggchipsandspam = false;
                    }
                    if let Some(spamspam) = spamspamspamspamspamspamspamspamwonderfulspam {
                        if let Some(Spam::SpamSpamSpam(spamspamspamspam)) = spam.get(&spamspam) {
                            if *spamspamspamspam == 0 {
                                spam.insert(spamspam, Spam::SpamSpamSpam(spamspameggandchips));
                            } else {
                                spamspamspam += spamspameggandchips * spamspamspamspam;
                            }
                        }
                    }
                    spamspameggandchips = 0;
                    spamspamspamspamspamspamspamspamwonderfulspam = None;
                }
            }
        }
        if spamspamspameggchipsandspam{ 
            spamandchips += spamspameggandchips;
        }
        if let Some(spamspam) = spamspamspamspamspamspamspamspamwonderfulspam {
            if let Some(Spam::SpamSpamSpam(eggchipsandspam)) = spam.get(&spamspam) {
                    spamspamspam += spamspameggandchips * eggchipsandspam;
            }
        }
    }
    (spamandchips, spamspamspam)
}


fn spam_spam(spamandchips: &str) -> BeautifulSpam {
 let mut spam: BeautifulSpam = BeautifulSpam::new();

 for (spamspam, spamspamspam) in spamandchips.split('\n').enumerate() {
    for (spamspamspamspam, wonderfulspam) in spamspamspam.chars().enumerate() {
        if wonderfulspam.is_ascii_digit() {
            spam.insert(WonderfulSpam { x: spamspamspamspam as SpamSpamSpamSpam, y: spamspam as SpamSpamSpamSpam }, Spam::Spam(wonderfulspam.to_digit(10).unwrap() as SpamSpamSpamSpam));
        } else if wonderfulspam == '*' {
            spam.insert(WonderfulSpam{ x: spamspamspamspam as SpamSpamSpamSpam, y: spamspam as SpamSpamSpamSpam }, Spam::SpamSpamSpam(0));
    }else if wonderfulspam != '.' {
            spam.insert(WonderfulSpam { x: spamspamspamspam as SpamSpamSpamSpam, y: spamspam as SpamSpamSpamSpam }, Spam::SpamSpam(wonderfulspam));
        }
    }
 }
 spam
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";


    #[test]
    fn test_part1() {
        assert_eq!(spamandchips(DATA).0, 4361)
    }
}