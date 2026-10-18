# Filler - Algorithmic Game Robot

An intelligent, high-performance algorithmic robot written in Rust for the **Filler** competition.

Filler is a 2-player grid-filling game where robots compete on the **Anfield** to capture the maximum territory. On each turn, the game engine supplies an Anfield map and a randomly shaped piece. The robot must place the piece such that **exactly one cell** overlaps its existing territory and **zero cells** overlap the opponent's territory, without exceeding the arena boundaries.

---

## 🏆 Key Features

- **Blazing Fast Performance**: Zero-dependency pure Rust implementation that evaluates every possible placement in under **1 millisecond** (far below the 10-second timeout limit).
- **Tactical BFS Heatmap AI**:
  - **Phase 1: Rapid Interception**: Uses multi-source BFS distance transforms from opponent positions to rush forward, spearheading toward the opponent before they can expand.
  - **Phase 2: Tactical Choke & Encirclement**: Prioritizes sealing off the opponent's frontier cells, boxing them into corners and cutting off their access to open board regions.
  - **Phase 3: Territory Maximization**: Once the opponent is immobilized, expands aggressively to maximize cell coverage.
- **Full Unit Test Suite**: Comprehensive tests validating:
  - Input parsing (Anfield grid dimensions, cell states, pieces)
  - Valid placement logic (single territory overlap, zero opponent overlap)
  - Boundary conditions (preventing any part of the piece shape from exceeding the arena)
  - Coordinate output format (`X Y\n`)
- **Cyberpunk Graphic Visualizer**: Interactive web visualizer with real-time canvas rendering, playback timeline, speed control, piece preview, and BFS heatmap overlay.

---

## 🚀 Quick Start

### 1. Run Unit Tests

From the repository root or inside the `solution/` directory:

```bash
cargo test
```

All unit tests will execute and pass:
```text
running 8 tests
test tests::test_boundary_conditions ... ok
test tests::test_coordinate_output ... ok
test tests::test_individual_parsers ... ok
test tests::test_input_parsing ... ok
test tests::test_no_moves_returns_zero_zero ... ok
test tests::test_player2_handling ... ok
test tests::test_strategy_selects_valid_move ... ok
test tests::test_valid_placement_logic ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; finished in 0.06s
```

### 2. Build the Robot

```bash
cargo build --release
```

The optimized binary is compiled to `target/release/solution` (or `solution/target/release/solution`).

---

## 🐳 Docker Setup & Running Matches

1. Download and extract the provided `filler.zip` if you haven't already:
   ```bash
   # docker build inside the docker_image directory
   cd docker_image
   docker build -t filler .
   ```

2. Run the interactive container with your solution mounted:
   ```bash
   docker run -v "$(pwd)/solution":/filler/solution -it filler
   ```

3. Inside the container, compile your robot:
   ```bash
   cd /filler/solution && cargo build --release
   cd /filler
   ```

4. Run matches against opponent robots:
   ```bash
   # Play against bender
   ./linux_game_engine -f maps/map01 -p1 solution/target/release/solution -p2 linux_robots/bender

   # Play as Player 2 against wall_e
   ./linux_game_engine -f maps/map01 -p1 linux_robots/wall_e -p2 solution/target/release/solution

   # Test against terminator
   ./linux_game_engine -f maps/map00 -p1 solution/target/release/solution -p2 linux_robots/terminator
   ```

---

## 🎨 Graphic Visualizer (Bonus)

Open `visualizer/index.html` in any web browser to launch the graphic visualizer:

- **Load Demo Battle**: Watch a simulated match showing tactical cut-off and encirclement.
- **Import Engine Log**: Paste or upload raw output from `./linux_game_engine` to replay and analyze matches.
- **Heatmap Overlay**: Toggle the AI's internal BFS distance calculations over the board in real time.
- **Playback Controls**: Step forward, step backward, auto-play, adjust speed from 0.5x to Max, and inspect cell coordinates on hover.

---

## 📂 Repository Structure

```text
├── solution/
│   ├── Cargo.toml          # Rust package manifest
│   └── src/
│       ├── main.rs         # Game loop & I/O handling
│       ├── board.rs        # Grid, player symbols & BFS heatmap
│       ├── piece.rs        # Piece representation & placement validator
│       ├── parser.rs       # Game engine protocol parser
│       ├── strategy.rs     # AI decision-making & tactical blocking
│       └── tests.rs        # Mandatory unit tests suite
├── visualizer/             # Graphic visualizer (Bonus)
│   ├── index.html
│   ├── style.css
│   └── app.js
├── Cargo.toml              # Workspace manifest
├── .gitignore
└── README.md
```
