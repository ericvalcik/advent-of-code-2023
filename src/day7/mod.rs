use std::cmp::Ordering;

mod consts;

pub fn count_winnings() -> u64 {
    let lines = consts::INPUT.trim().lines();
    let mut games: Vec<Game> = Vec::new();
    for line in lines {
        games.push(get_game(line));
    }
    games.sort_by(cmp_games);
    let mut winnings: u64 = 0;
    for (i, game) in games.iter().enumerate() {
        winnings += (i as u64 + 1) * game.bid;
    }
    winnings
}

#[derive(Debug)]
struct Game {
    cards: String,
    bid: u64
}

fn cmp_games(a: &Game, b: &Game) -> Ordering {
    let a_rank = get_card_rank(a);
    let b_rank = get_card_rank(b);
    if a_rank == b_rank {
        let mut b_cards_list = b.cards.chars().map(|c| get_card_num(c));
        for a_card in a.cards.chars().map(|c| get_card_num(c)) {
            let b_card = b_cards_list.next().unwrap();
            match a_card.cmp(&b_card) {
                Ordering::Equal => continue,
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
            }
        }
        return Ordering::Equal;
    }
    if a_rank > b_rank {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Rank {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPairs = 3,
    OnePair = 2,
    HighCard = 1
}

fn get_card_rank(game: &Game) -> Rank {
    let mut cards = game_to_map(game);
    let jokers = cards[0];
    cards[0] = 0;
    let max_cards_at = cards.iter().enumerate().max_by_key(|&(_, x)| x).unwrap().0;
    let snd_max_cards_at = cards.iter().enumerate().max_by_key(|&(i, x)| if i == max_cards_at { &0 } else { x }).unwrap().0;
    if jokers + cards[max_cards_at] == 5 {
        return Rank::FiveOfAKind;
    }
    if jokers + cards[max_cards_at] == 4 {
        return Rank::FourOfAKind;
    }
    if (jokers + cards[max_cards_at] == 3 && cards[snd_max_cards_at] == 2 ||
        jokers + cards[max_cards_at] == 2 && cards[snd_max_cards_at] == 3 ||
        jokers + cards[snd_max_cards_at] == 2 && cards[max_cards_at] == 3 ||
        jokers + cards[snd_max_cards_at] == 3 && cards[max_cards_at] == 2) {
        return Rank::FullHouse;
    }
    if jokers + cards[max_cards_at] == 3 {
        return Rank::ThreeOfAKind;
    }
    if  jokers + cards[snd_max_cards_at] == 2 && cards[max_cards_at] == 2 {
        return Rank::TwoPairs;
    }
    if jokers + cards[max_cards_at] == 2 {
        return Rank::OnePair;
    }
    Rank::HighCard
}

fn game_to_map(game: &Game) -> [u8; 13] {
    let mut map: [u8; 13] = [0; 13];
    for card in game.cards.chars() {
        let card_num = get_card_num(card);
        map[card_num] += 1;
    }
    map
}

fn get_card_num(card: char) -> usize {
    match card {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("Invalid card: {card}")
    }
}

fn get_game(line: &str) -> Game {
    let mut words = line.trim().split_whitespace();
    let cards = words.next().unwrap().to_string();
    let bid = words.next().unwrap().parse::<u64>().unwrap();
    Game {
        cards,
        bid,
    }
}