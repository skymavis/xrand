const n = 25
const m = 7
const s = 7
const t = 15
const a = 0x8ebfd028
const b = 0x2b5b2500
const c = 0xdb8b0000

interface ITGFSR {
  next: () => number
  random: () => number
}

export class TGFSR implements ITGFSR {
  private x: number[] = new Array(n).fill(0)
  private k: number = 0
  private initialized: boolean = false

  constructor(seed: number) {
    this.initialize(seed)
  }

  private initialize(seed: number): void {
    for (let i = 0; i < n; i++) {
      seed &= 0xffffffff
      this.x[i] = seed >>> 0
      seed = (seed * 1313 + 88897) & 0xffffffff
    }

    this.k = n - 1
    this.initialized = true
  }

  private iterate(): void {
    for (let i = 0; i < n - m; i++) {
      this.x[i] = (this.x[i + m] ^ (this.x[i] >>> 1) ^ (this.x[i] & 1 ? a : 0)) >>> 0
    }

    for (let i = n - m; i < n; i++) {
      this.x[i] = (this.x[i + m - n] ^ (this.x[i] >>> 1) ^ (this.x[i] & 1 ? a : 0)) >>> 0
    }
  }

  public next(): number {
    if (!this.initialized) {
      this.initialize(1)
    }

    this.k++
    if (this.k === n) {
      this.iterate()
      this.k = 0
    }

    let y = this.x[this.k] ^ ((this.x[this.k] << s) & b)
    y ^= (y << t) & c

    return y >>> 0
  }

  public random(): number {
    return this.next() & 0x7fffffff
  }
}
