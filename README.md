[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-24ddc0f5d75046c5622901739e7c5dd533143b0c8e959d652212380cedb1ea36.svg)](https://classroom.github.com/a/RQfdh2iK)
# BlackJack

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/rustvu/rustwc_ci/ci.yml)

## Description

In this coding assignment, I will develop a console-based implementation of the classic card game, Blackjack, also known as 21. The objective of this project is to create a functional Blackjack game, while also incorporating betting features. The focus will be on the core gameplay mechanics, ensuring that the game accurately follows the rules of Blackjack. In order to do that I will need to implement a virtual deck of 52 playing cards, using a data structure to represent the deck and ensure that cards are drawn and shuffled appropriately. I will need to develop classes  to represent the player and dealer, keep track of their hands (initially two cards for each) and calculate their hand values. In addition to that I must implement the player's ability to "hit" (draw a card) or "stand" (keep their current hand), ensuring that the player can make these decisions during their turn. The hardest part of this project will be  implementing the dealer's logic to hit or stand based on the standard Blackjack rules (e.g., hitting until the hand value is at least 17). Finally, I need to determine when a game round ends, and declare the winner or a tie along with handling scenarios like player and dealer "busting" (going over 21) appropriately.

## Installation

Make sure you have Rust and Cargo installed on your system. Next, copy the project link and clone the repository using git clone. In the terminal, you would navigate into the folder you want the project to go in and then write: git clone https://github.com/rustvu-2023f/project-leblanbe.git. To run the project you will use the cargo build and run command. You will need to input some information when playing the game. Enjoy!

## How to use
The game will display directions if you would like to see them. It will then prompt you to place a bet. Enter the desired bet amount when prompted, you cannot place a bet less than or equal to 0. After the deck is shuffled, you will be dealt two initial cards. Follow the on-screen instructions to choose whether to "hit" or "stand" during your turn. Type your choice and press Enter. If you choose to "hit," a card will be drawn, and the total value of your hand will be updated. If the total value of your hand exceeds 21, you'll bust (meaning the dealer wins the round), and the game will progress to the next round. Once you choose to "stand" or bust, it will be the dealer's turn. The dealer will draw cards until their hand reaches a total value of 17 or higher. The winner will be determined based on the total values of the hands, and the bet will be adjusted accordingly. After a round is complete, the game will ask if you want to play another round. Type "yes" to play another round or "no" to exit the game. If you choose not to play another round, the game will display your final balance and exit. You can always run the game again to start a new session.

## Example Scenarios
Each game begins with the player and the dealer each receiving two cards.

Scenario 1: Player Wins
Player's hand: 7 of Hearts, King of Spades (total value: 17)
Dealer's face-up card: 9 of Clubs
Player decides to "stand."
The dealer reveals their face-down card: 6 of Diamonds, resulting in a hand value of 15.
The dealer is forced to "hit" as their hand value is less than 17.
The dealer draws a 10 of Diamonds, resulting in a total hand value of 25 (bust).
The player wins the round because the dealer busts.

Scenario 2: Player Busts
Player's hand: 10 of Spades, 8 of Hearts (total value: 18)
Dealer's face-up card: 2 of Clubs
Player decides to "hit."
The player draws a 5 of Diamonds, resulting in a total hand value of 23 (bust).
The dealer wins the round because the player busts.

Scenario 3: Player Gets Blackjack
Player's hand: Ace of Hearts, King of Spades (total value: 21, Blackjack!)
Dealer's face-up card: 7 of Diamonds
The player wins the round with a Blackjack, as the dealer's face-down card doesn't matter.

Scenario 4: Push (Tie)
Player's hand: 8 of Clubs, 9 of Diamonds (total value: 17)
Dealer's face-up card: Ace of Spades
Player decides to "stand."
The dealer reveals their face-down card: 7 of Hearts, resulting in a hand value of 18.
The dealer wins the round because their hand value is higher.
It's also possible to have a tie (push) if the dealer and player both have the same hand value.
