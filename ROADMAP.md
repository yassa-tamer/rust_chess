# Rust Chess — Improvement Roadmap

A step-by-step plan to improve this chess CLI, ordered for a developer getting familiar with the codebase and learning Rust progressively.

## Phase 1 — Get familiar with the codebase

### Fix pawn promotion logic ([#5](https://github.com/AbanoubGhadban/rust_chess/issues/5))

Pawn promotion is broken. The current code takes a captured/dead piece and places it back on the board. The correct behavior is to let the player choose a new piece (Queen, Rook, Bishop, or Knight) regardless of captures.

**Files:** `chessboard.rs`, `game.rs`, `board_manager.rs`, `ui/cmd.rs`, `errors.rs`
**Rust concepts:** enum construction, `match`, factory-style creation

### Implement Display trait for pieces and board ([#6](https://github.com/AbanoubGhadban/rust_chess/issues/6))

Replace the `Presenter` module with idiomatic `fmt::Display` implementations so that `println!("{board}")` and `piece.to_string()` just work.

**Files:** `pieces/piece.rs`, `chessboard.rs`, `presenters/cmd.rs`
**Rust concepts:** `Display` trait, `fmt::Formatter`, `write!` macro, `Debug` vs `Display`

## Phase 2 — Medium features, meet the borrow checker

### Add game state machine and quit command ([#7](https://github.com/AbanoubGhadban/rust_chess/issues/7))

The game loop runs forever with no exit condition. Add a `GameState` enum (`Playing`, `Check`, `Checkmate(Color)`, `Stalemate`, `Draw(DrawReason)`) and wire it into the game loop. Add quit/resign commands.

**Files:** `game.rs`, `ui/cmd.rs`, `chessboard.rs`
**Rust concepts:** enums with associated data, exhaustive `match`, state machine pattern

### Implement checkmate and stalemate detection ([#4](https://github.com/AbanoubGhadban/rust_chess/issues/4))

The game detects check but not checkmate or stalemate. After each move, check if the opponent has any legal move. Clone the board to simulate candidate moves without mutating the real board.

**Files:** `chessboard.rs`, `board_manager.rs`, `ui/cmd.rs`
**Rust concepts:** `Clone` vs `Copy`, borrow checker (can't mutate while reading), simulation via cloning

## Phase 3 — Ownership mastery

### Implement move history with undo support ([#8](https://github.com/AbanoubGhadban/rust_chess/issues/8))

No record of moves exists. Design a `MoveRecord` struct (piece, from, to, captured piece, move type) and maintain a `Vec<MoveRecord>` stack. The ownership challenge: when you capture a piece, the history must own it so undo can restore it.

**Files:** `board_manager.rs`, `chessboard.rs`, `game.rs`, `ui/cmd.rs`, new `move_record.rs`
**Rust concepts:** ownership transfer, `Option<Piece>`, `Vec` as a stack (`push`/`pop`), command pattern
**Blocks:** castling, en passant fix, threefold repetition

### Implement castling ([#9](https://github.com/AbanoubGhadban/rust_chess/issues/9))

The most complex special move: two pieces move simultaneously with 5 preconditions (neither piece moved, no pieces between, king not in check, doesn't pass through check, doesn't land in check). Depends on move history to check "has this piece ever moved?"

**Files:** `pieces/king.rs`, `board_manager.rs`, `chessboard.rs`, `pieces/types/move_direction.rs`
**Rust concepts:** complex validation chains, extending enums, nested borrows
**Depends on:** move history

### Fix en passant temporal validation ([#10](https://github.com/AbanoubGhadban/rust_chess/issues/10))

En passant is structurally implemented but temporally broken — it's allowed at any time instead of only immediately after the opponent's 2-square pawn push. Fix by checking `last_move()` from move history.

**Files:** `board_manager.rs`, `tests/board_manager_tests.rs`
**Rust concepts:** `Vec::last()` returning `Option<&T>`, pattern matching on structs
**Depends on:** move history

## Phase 4 — Algorithms

### Implement threefold repetition detection with Zobrist hashing ([#11](https://github.com/AbanoubGhadban/rust_chess/issues/11))

Draw rule: same board position 3 times = draw. Use Zobrist hashing for O(1) position comparison — assign random `u64` to each (piece, square) pair, XOR them together. Incrementally update the hash on each move.

**Files:** new `zobrist.rs`, `board_manager.rs` or `game.rs`
**Rust concepts:** `HashMap<u64, u8>`, `rand` crate (first external dependency), bitwise XOR, `Hash` trait
**Depends on:** move history

### Implement AI opponent with minimax and parallel search ([#12](https://github.com/AbanoubGhadban/rust_chess/issues/12))

The capstone. Add a single-player mode with an AI that uses minimax + alpha-beta pruning. Parallelize the root-level search with `std::thread::spawn` — each candidate move evaluated on its own thread.

**Files:** new `ai/` module (`evaluator.rs`, `minimax.rs`), `game.rs`, `ui/cmd.rs`
**Rust concepts:** `std::thread::spawn`, `mpsc::channel`, `Arc<T>`, `Send` + `Sync` traits, `move` closures, recursion
**Depends on:** checkmate/stalemate detection

## Completed

- [x] Fix input crashes, add typed error handling, and apply idiomatic Rust patterns ([PR #3](https://github.com/AbanoubGhadban/rust_chess/pull/3))
