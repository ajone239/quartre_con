Quatre Con
====================

In this repo is defined a console Connect 4 remake.
This console game has a TUI interface and a bot you can play against.

The bot implements raw minimax as well as minimax with alpha-beta pruning.
The perform the same in this instance as they both return the best move they find.
However, you can see how much less the alpha-beta implementation has to evaluate nodes.
There is also some remedial threat detection in the evaluation function.
This is to say: the lowest threat on the row is rated by the bot and stacked threats are very well rated by the bot.

Play
--------------------

The game is fronted by some CLI args and then your game begins.
An example to make two bots fight would be: `quatre_con -o bot -t bot`
An example to fight an opponent who plays randomly: `quatre_con -t random`

### Usage

```
Usage: quatre_con [OPTIONS]

Options:
  -o, --one-player <ONE_PLAYER>
          The type of player player1 will be [default: human]
      --one-player-alg <ONE_PLAYER_ALG>
          The alg for player1 [default: alphabeta]
      --one-player-depth <ONE_PLAYER_DEPTH>
          The depth for player1 3 is easy 8 is impossible [default: 5]
  -t, --two-player <TWO_PLAYER>
          The type of player player2 will be [default: bot]
      --two-player-alg <TWO_PLAYER_ALG>
          The alg for player2 [default: alphabeta]
      --two-player-depth <TWO_PLAYER_DEPTH>
          The depth for player2 3 is easy 8 is impossible [default: 5]
  -h, --help
          Print help
  -V, --version
          Print version
```
