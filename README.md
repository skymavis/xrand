# xrand

Multi-language implementation of the Twisted Generalized Feedback Shift Register ([TGFSR](https://dl.acm.org/doi/pdf/10.1145/189443.189445)) pseudorandom number generator

## About

![xrand banner](/assets/banner.webp)

`xrand` is a pseudorandom number generator based on the Twisted Generalized Feedback Shift Register ([TGFSR](https://dl.acm.org/doi/pdf/10.1145/189443.189445)) algorithm. Designed for developers across various ecosystems, `xrand` delivers consistent, high-quality random numbers in **Python**, **TypeScript**, **Go**, and **Rust**. Ready for use in games, simulations, or other applications that require reliable randomness.


#### Examples

[[Typescript]()] [[Python]()] [[Rust]()] [[Go]()]


## Instalation (TBU)

### Typescript

```bash
bun i @skymavis/xrand
```

### Python

```bash
pip install xrand
```

### Go

```bash
go get github.com/maxvis-x/xrand
```

### Rust

Add this to your `Cargo.toml`:

```bash
[dependencies]
xrand = "0.1.0"
```

## Usage

### Typescript

```ts
import { TGFSR } from "xrand";

// Example: Random shuffle for a card game
const generator = new TGFSR(12345);
const cards = ["Ace", "King", "Queen", "Jack"];
cards.sort(() => (generator.next() % 2) - 0.5);
console.log("Shuffled cards:", cards);
```

### Python

```python
from xrand import TGFSR

# Example: Random item drop for a game
generator = TGFSR(seed=12345)
item = ["sword", "shield", "potion"]
print("Item drop:", item[generator.next() % len(item)])
```

### Go

```go
package main

import (
    "fmt"
    "github.com/skymavis/xrand"
)

// Example: Random delays for an app animation
func main() {
    generator := xrand.NewTGFSR(12345)
    fmt.Printf("Random delay: %dms\n", generator.Next()%500+100)
}
```

### Rust

Add this to your `Cargo.toml`:

```rust
use xrand::TGFSR;

// Example: Procedural generation for a game
fn main() {
    let mut generator = TGFSR::new(12345);
    let terrain = ["grass", "water", "mountain"];
    println!("Generated terrain: {}", terrain[generator.next() as usize % terrain.len()]);
}
```

## Use Cases

### Axie Classic Raffle event

In Axie Classic Season 6, the Raffle event used `xrand` to pick winners based on their ticket number.

[[Visit site](https://classic-raffle.axiedao.org/)]
![Axie Classic Raffle](/assets/usecase-classic.webp)

### Nio's Nightmare Raffle event

[[Visit site](https://nioraffle.axiedao.org/)]
![Nio's Nightmare Raffle](/assets/usecase-nio.webp)
