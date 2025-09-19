use rand::seq::SliceRandom;
use rand::thread_rng;

// Define a structure for a card.

#[derive(Debug, PartialEq)]
struct Card {
    rank: &'static str, // Card rank (e.g., "2", "King")
    suit: &'static str, // Card suit (e.g., "Hearts", "Spades")
    value: i32,         // Card value in Blackjack (e.g., 2-10, 10 for face cards)
}

// Define a structure for a player's hand.
struct Player {
    cards: Vec<Card>, // Cards in the hand
    total_value: i32, // Total value of the hand
    balance: i32,     // Player's current balance
}

// Function to create and shuffle a deck of cards(52).
fn create_shuffled_deck() -> Vec<Card> {
    let ranks = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace",
    ];
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];

    // Create an empty deck to store the cards.
    let mut deck: Vec<Card> = Vec::new();

    // Create the deck by combining ranks, suits, and values.
    for suit in suits.iter() {
        for (value, rank) in ranks.iter().enumerate() {
            let card = Card {
                rank,
                suit,
                value: match value {
                    0..=8 => value as i32 + 2, // Cards 2-10 have their face value.
                    9..=11 => 10,              // Face cards (Jack, Queen, King) have a value of 10.
                    12 => 11,                  // Ace can be 11 initially.
                    _ => 0,
                },
            };
            deck.push(card);
        }
    }

    // Shuffle the deck using the rand crate.
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);

    deck
}

// Function to deal a card from the deck to a player's hand.
fn deal_card(hand: &mut Player, deck: &mut Vec<Card>) {
    //Because the dec has been shuffled, we can pop the last card from the deck and add it to the hand instead of drawing a card randomly
    if let Some(card) = deck.pop() {
        hand.cards.push(card);
    }
}

// Function to calculate the total value of a hand.
fn calculate_hand_value(hand: &mut Player) -> i32 {
    let mut total_value = 0;
    let mut has_ace = false;

    for card in hand.cards.iter() {
        total_value += card.value;

        // Check for Aces and handle their flexible value.
        if card.rank == "Ace" {
            has_ace = true;
        }
    }

    // If the hand has an Ace and its value is 11, but the total value is over 21, change the Ace's value to 1.
    if has_ace && total_value > 21 {
        total_value -= 10;
    }
    hand.total_value = total_value;
    total_value
}

fn place_bet(player: &mut Player) -> i32 {
    loop {
        println!("Your current balance: ${}", player.balance);
        println!("Enter your bet amount: ");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Parse the input as an integer.
        match input.trim().parse::<i32>() {
            Ok(bet) => {
                if bet > player.balance {
                    println!("You cannot bet more than your balance. Try again.");
                } else if bet <= 0 {
                    println!("Invalid bet amount. Please enter a positive value.");
                } else {
                    println!("You placed a bet of ${}", bet);
                    return bet;
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }
}

// Function for a player's turn (hit or stand).
fn player_turn(player_hand: &mut Player, deck: &mut Vec<Card>) {
    loop {
        println!("Player's hand: {:?}", player_hand.cards);
        let total_value = calculate_hand_value(player_hand);
        println!("Total value: {}", total_value);

        if total_value >= 21 {
            // Player has reached 21 or busted, so their turn ends.
            println!("Player stands.");
            break;
        }

        println!("Do you want to 'hit' or 'stand'?");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let choice = input.trim().to_lowercase();

        if choice == "hit" {
            // Player chooses to hit, so draw a card from the deck and add it to their hand.
            deal_card(player_hand, deck);

            // Check if the player has busted (exceeded 21).
            if calculate_hand_value(player_hand) > 21 {
                println!("Player's hand: {:?}", player_hand.cards);
                println!("Player busts!");
                break;
            }
        } else if choice == "stand" {
            // Player chooses to stand, ending their turn.
            println!("Player stands.");
            break;
        } else {
            println!("Invalid choice. Please enter 'hit' or 'stand'.");
        }
    }
}

// Function for the dealer's turn (hit or stand according to rules).
fn dealer_turn(dealer_hand: &mut Player, deck: &mut Vec<Card>) {
    loop {
        let total_value = calculate_hand_value(dealer_hand);

        if total_value >= 17 {
            // Dealer's hand value is at least 17; their turn ends.
            break;
        }

        // Dealer hits (draws a card from the deck).
        deal_card(dealer_hand, deck);
    }
}

// Function to determine the winner of the game.
fn determine_winner(player: &mut Player, dealer: &mut Player, bet: i32) {
    let player_total = calculate_hand_value(player);
    let dealer_total = calculate_hand_value(dealer);

    if (player_total <= 21 && player_total > dealer_total)
        || (dealer_total > 21 && player_total <= 21)
    {
        // Player wins the round.
        let winnings = bet * 2; // Double the bet as winnings.
        player.balance = player.balance + winnings;
        println!("Player wins ${}!", winnings);
    } else if player_total == dealer_total {
        // It's a tie (push), the player gets their bet back.
        println!("It's a tie. The bet is returned to the player.");
    } else {
        // Dealer wins the round.
        println!("Dealer wins. Player loses ${}!", bet);
        player.balance = player.balance - bet;
    }
    display_balance(player);
}

fn display_balance(player: &Player) {
    println!("Current balance: ${}", player.balance);
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]
    fn test_create_shuffled_deck() {
        let original_deck = create_shuffled_deck();
        let shuffled_deck = create_shuffled_deck();

        // Check if both decks contain the same length of cards.
        assert_eq!(original_deck.len(), shuffled_deck.len());

        // Ensure that the shuffled deck is not equal to the original deck.
        assert_ne!(original_deck, shuffled_deck);
    }

    #[test]
    fn test_deal_card() {
        let mut deck = create_shuffled_deck();
        let mut hand = Player {
            cards: Vec::new(),
            total_value: 0,
            balance: 0,
        };

        // Deal a card to the hand.
        deal_card(&mut hand, &mut deck);

        // Ensure that the hand now contains one card.
        assert_eq!(hand.cards.len(), 1);

        // Ensure that the card is removed from the deck.
        assert_eq!(deck.len(), create_shuffled_deck().len() - 1);

        // Deal another card to the hand.
        deal_card(&mut hand, &mut deck);

        // Ensure that the hand now contains two cards.
        assert_eq!(hand.cards.len(), 2);

        // Ensure that the card is removed from the deck.
        assert_eq!(deck.len(), create_shuffled_deck().len() - 2);
    }

    #[test]
    fn test_calculate_hand() {
        // Test a hand with numerical cards.
        let mut hand = Player {
            cards: vec![
                Card {
                    rank: "2",
                    suit: "Hearts",
                    value: 2,
                },
                Card {
                    rank: "5",
                    suit: "Spades",
                    value: 5,
                },
                Card {
                    rank: "8",
                    suit: "Diamonds",
                    value: 8,
                },
            ],
            total_value: 0,
            balance: 10,
        };
        //ensures that the returned value is correct
        assert_eq!(calculate_hand_value(&mut hand), 15);

        //ensures that the hand's total value is updated
        assert_eq!(hand.total_value, 15);

        // Test a hand with face cards.
        let mut hand2 = Player {
            cards: vec![
                Card {
                    rank: "King",
                    suit: "Clubs",
                    value: 10,
                },
                Card {
                    rank: "Queen",
                    suit: "Hearts",
                    value: 10,
                },
                Card {
                    rank: "Jack",
                    suit: "Spades",
                    value: 10,
                },
            ],
            total_value: 0,
            balance: 10,
        };
        assert_eq!(calculate_hand_value(&mut hand2), 30);
        assert_eq!(hand2.total_value, 30);

        // Test a hand with an Ace.
        let mut hand3 = Player {
            cards: vec![
                Card {
                    rank: "Ace",
                    suit: "Diamonds",
                    value: 11,
                },
                Card {
                    rank: "9",
                    suit: "Hearts",
                    value: 9,
                },
            ],
            total_value: 0,
            balance: 10,
        };
        assert_eq!(calculate_hand_value(&mut hand3), 20);
        assert_eq!(hand3.total_value, 20);

        // Test a hand with multiple Aces.
        let mut hand4 = Player {
            cards: vec![
                Card {
                    rank: "Ace",
                    suit: "Clubs",
                    value: 11,
                },
                Card {
                    rank: "Ace",
                    suit: "Spades",
                    value: 11,
                },
                Card {
                    rank: "6",
                    suit: "Hearts",
                    value: 6,
                },
            ],
            total_value: 0,
            balance: 10,
        };
        assert_eq!(calculate_hand_value(&mut hand4), 18);
        assert_eq!(hand4.total_value, 18);
    }

    // #[test]
    //     fn test_betting(){
    //         //I tested this method by manually imputing these values into the terminal.

    //         let mut player = Player { cards: Vec::new(), total_value: 0, balance: 100 };

    //         // Test a valid bet.
    //         let bet = place_bet(&mut player);
    //         assert_eq!(bet, 10);
    //         assert_eq!(player.balance, 90);

    //         // Test a bet that is equivalent to the player's balance.
    //         let bet = place_bet(&mut player);
    //         assert_eq!(bet, 90);
    //         assert_eq!(player.balance, 0);

    //         // Test a bet that is less than or equal to 0.
    //         let bet = place_bet(&mut player);
    //         assert_eq!(bet, 0);
    //         assert_eq!(player.balance, 0);

    //         // Test a bet that is greater to the player's balance.
    //         let bet = place_bet(&mut player);
    //         assert_eq!(bet, 100);
    //         assert_eq!(player.balance, 0);

    //     }

    // #[test]
    //     fn test_player_turn(){
    //                 // Test a hand with face cards.
    //         let mut player = Player {
    //             cards: vec![
    //                 Card { rank: "King", suit: "Clubs", value: 10 },
    //                 Card { rank: "Queen", suit: "Hearts", value: 10 },
    //             ],
    //             total_value: 0,
    //             balance: 10,
    //         };

    //         let mut deck = create_shuffled_deck();

    //         //I tested this manually by inputting the values to give me the following results
    //         //1. A situation where the player continually hits and eventually busts
    //         //2. A situation where the player hits once and stands
    //         //3. A situation where the player stands on the first choice
    //         //4. The player submits and invalid choice.
    //         player_turn(&mut player, & mut dealer);

    //     }

    #[test]
    fn test_dealer_turn() {
        let mut shuffled_deck = create_shuffled_deck();
        // Test the dealer's turn with a hand that needs to hit.
        let mut dealer_hand = Player {
            cards: vec![
                Card {
                    rank: "10",
                    suit: "Hearts",
                    value: 10,
                },
                Card {
                    rank: "6",
                    suit: "Spades",
                    value: 6,
                },
            ],
            total_value: 0,
            balance: 100,
        };
        dealer_turn(&mut dealer_hand, &mut shuffled_deck);

        assert!(dealer_hand.total_value >= 17);

        // Test the dealer's turn with a hand that should stand.
        let mut dealer_hand = Player {
            cards: vec![
                Card {
                    rank: "10",
                    suit: "Hearts",
                    value: 10,
                },
                Card {
                    rank: "7",
                    suit: "Spades",
                    value: 7,
                },
            ],
            total_value: 0,
            balance: 100,
        };
        dealer_turn(&mut dealer_hand, &mut shuffled_deck);
        assert_eq!(dealer_hand.total_value, 17);

        // Test the dealer's turn with a hand that already meets the threshold.
        let mut dealer_hand = Player {
            cards: vec![
                Card {
                    rank: "10",
                    suit: "Hearts",
                    value: 10,
                },
                Card {
                    rank: "8",
                    suit: "Spades",
                    value: 8,
                },
            ],
            total_value: 0,
            balance: 100,
        };
        dealer_turn(&mut dealer_hand, &mut shuffled_deck);
        assert_eq!(dealer_hand.total_value, 18);
    }

    #[test]
    fn test_determine_winner() {
        // Test a scenario where the player wins.
        let mut player = Player {
            cards: vec![
                Card {
                    rank: "10",
                    suit: "Hearts",
                    value: 10,
                },
                Card {
                    rank: "8",
                    suit: "Spades",
                    value: 8,
                },
            ],
            total_value: 0,
            balance: 100,
        };
        let mut dealer = Player {
            cards: vec![
                Card {
                    rank: "10",
                    suit: "Diamonds",
                    value: 10,
                },
                Card {
                    rank: "7",
                    suit: "Clubs",
                    value: 7,
                },
            ],
            total_value: 0,
            balance: 100,
        };

        let bet = 50;
        determine_winner(&mut player, &mut dealer, bet);
        assert_eq!(player.balance, 200);

        // Test a scenario where the dealer wins.
        let mut player = Player {
            cards: vec![
                Card {
                    rank: "10",
                    suit: "Hearts",
                    value: 10,
                },
                Card {
                    rank: "8",
                    suit: "Spades",
                    value: 8,
                },
            ],
            total_value: 0,
            balance: 100,
        };
        let mut dealer = Player {
            cards: vec![
                Card {
                    rank: "10",
                    suit: "Diamonds",
                    value: 10,
                },
                Card {
                    rank: "King",
                    suit: "Clubs",
                    value: 10,
                },
            ],
            total_value: 0,
            balance: 100,
        };
        let bet = 50;
        determine_winner(&mut player, &mut dealer, bet);
        assert_eq!(player.balance, 50);

        // Test a scenario where it's a tie (push).
        let mut player = Player {
            cards: vec![
                Card {
                    rank: "10",
                    suit: "Hearts",
                    value: 10,
                },
                Card {
                    rank: "7",
                    suit: "Spades",
                    value: 7,
                },
            ],
            total_value: 0,
            balance: 100,
        };
        let mut dealer = Player {
            cards: vec![
                Card {
                    rank: "10",
                    suit: "Diamonds",
                    value: 10,
                },
                Card {
                    rank: "7",
                    suit: "Clubs",
                    value: 7,
                },
            ],
            total_value: 0,
            balance: 100,
        };

        let bet = 50;
        determine_winner(&mut player, &mut dealer, bet);
        assert_eq!(player.balance, 100); // in a tie scenario the bet is returned.

        //Player busts
        let mut player = Player {
            cards: vec![
                Card {
                    rank: "10",
                    suit: "Hearts",
                    value: 10,
                },
                Card {
                    rank: "10",
                    suit: "Diamons",
                    value: 10,
                },
                Card {
                    rank: "10",
                    suit: "Clubs",
                    value: 10,
                },
            ],
            total_value: 0,
            balance: 100,
        };
        let mut dealer = Player {
            cards: vec![
                Card {
                    rank: "10",
                    suit: "Spades",
                    value: 10,
                },
                Card {
                    rank: "7",
                    suit: "Clubs",
                    value: 7,
                },
            ],
            total_value: 0,
            balance: 100,
        };

        let bet = 50;
        determine_winner(&mut player, &mut dealer, bet);
        assert_eq!(player.balance, 50);

        //Dealer busts
        let mut player = Player {
            cards: vec![
                Card {
                    rank: "10",
                    suit: "Diamonds",
                    value: 10,
                },
                Card {
                    rank: "7",
                    suit: "Clubs",
                    value: 7,
                },
            ],
            total_value: 0,
            balance: 100,
        };
        let mut dealer = Player {
            cards: vec![
                Card {
                    rank: "8",
                    suit: "Hearts",
                    value: 8,
                },
                Card {
                    rank: "8",
                    suit: "Diamons",
                    value: 8,
                },
                Card {
                    rank: "10",
                    suit: "Hearts",
                    value: 10,
                },
            ],
            total_value: 0,
            balance: 100,
        };

        let bet = 50;
        determine_winner(&mut player, &mut dealer, bet);
        assert_eq!(player.balance, 200);
    }
}

fn main() {
    println!("Welcome to Blackjack!");
    println!("Would you like to know the rules of the game? (yes/no)");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let choice = input.trim().to_lowercase();

    if choice != "yes" && choice != "no" {
        loop {
            println!("Invalid input, would you like to know the rules of the game? (yes/no)");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            let choice = input.trim().to_lowercase();
            if choice == "yes" {
                println!("Blackjack Rules:");
                println!(
                    "1. The goal of the game is to beat the dealer's hand without going over 21."
                );
                println!(
                    "2. The player is initially dealt two cards, and the dealer is dealt one card."
                );
                println!("3. Cards 2-10 are worth their face value, face cards (King, Queen, Jack) are worth 10, and Aces can be worth 1 or 11.");
                println!("4. The player can 'hit' to draw additional cards or 'stand' to end their turn.");
                println!("5. The dealer must hit until their hand's value is at least 17.");
                println!("6. If the player's hand value exceeds 21, they bust and lose.");
                println!(
                    "7. If the dealer busts or the player's hand is closer to 21, the player wins."
                );
                println!(
                    "8. If the player's and dealer's hands have the same value, it's a tie (push)."
                );
                println!("9. You can place bets at the beginning of each round. If you win, you gain double your betting amount. If you lose, you lose your betting amount.");
                println!();
                break;
            }
            if choice == "no" {
                break;
            }
        }
    }

    // Create a player and a dealer with an initial balance.
    let mut player = Player {
        cards: Vec::new(),
        total_value: 0,
        balance: 100,
    };
    let mut dealer = Player {
        cards: Vec::new(),
        total_value: 0,
        balance: 0,
    };

    loop {
        // Display the player's current balance.
        display_balance(&player);

        //Game ends if player is out of betting "money"
        if player.balance <= 0 {
            println!("You're out of money. Game over!");
            break;
        }

        // Simulate the player placing a bet.
        let bet = place_bet(&mut player);

        // Initialize and shuffle the deck of cards after every round.
        let mut deck = create_shuffled_deck();

        player.cards.clear();
        // Deal the initial cards to the player and dealer.
        deal_card(&mut player, &mut deck);
        deal_card(&mut player, &mut deck);

        deal_card(&mut dealer, &mut deck);

        // Update the initial total value of the hands.
        calculate_hand_value(&mut player);
        calculate_hand_value(&mut dealer);

        // Simulate the player's turn and the dealer's turn.
        player_turn(&mut player, &mut deck);
        dealer_turn(&mut dealer, &mut deck);

        // Determine the winner and update the player's balance based on the bet.
        determine_winner(&mut player, &mut dealer, bet);

        // Ask the player if they want to play another round.
        println!("Do you want to play another round? (yes/no)");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let choice = input.trim().to_lowercase();

        if choice != "yes" {
            println!(
                "Thanks for playing! Your final balance: ${}",
                player.balance
            );
            break;
        }
    }
}
