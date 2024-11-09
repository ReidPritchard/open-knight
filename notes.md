# Chess Move Analysis

## Move Sequences

- Main line: e4 e5 2. Nf3 Nc6 3. Bb5 a6 4. Ba4 Nf6 5. O-O Be7 6. Re1 b5
- Variation: ~~e4 e5 2. Nf3 Nc6~~ 3. Bdxc6 a6 4. c3 Bc5 5. d4 exd4 6. cxd4 Nbd7 7. c5

## Line "Split" Logic

Ideally we don't want to keep track of the entire line, but only when it splits into variations.

### Main Line

- Move Number: 5
- Move SAN: Bb5

### Variation 1

- Move Number: 5
- Move SAN: Bdxc6

## Variation Implementation Notes

1. Back-references approach (linked list)

   - Each move references previous move (move <- prev_move)
   - "Split" moves need a ranking or priority to determine the order to display the variations
     - variation_order?

```
a -> b -> c -> d

a -> e -> f -> d

a         d -> y
|         |
b -> e -> |
e -> b -> |

a           d -> y
|           |
1b -> 1e -> |
2e -> 2b -> |
```

2. Graph approach

Positions are nodes and moves are the edges between them.

Node:

- FEN
- Parents (back-references)
- Children (forward-references)
- Annotations

Edge (Move):

- Game ID
  - Since a set of moves make up a game.
- Move SAN
- Move Number (half-move number)?
- Parent Node
- Child Node
- Annotations

Annotations can be related to the move or the position.

- Move annotations:

  - Best?
  - NAG?
  - Comment?
  - Centipawn Loss?

- Position annotations:
  - Evaluation(s)?
  - Arrows?
  - Circles?
  - Comments?
  - Motifs?
  - Tactics?
