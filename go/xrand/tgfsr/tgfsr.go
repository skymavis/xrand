package tgfsr

type TGFSR struct {
    x           [25]uint32
    k           int
    initialized bool

    // Constants
    n int
    m int
    s int
    t int
    a uint32
    b uint32
    c uint32
}

func New() *TGFSR {
    return &TGFSR{
        n: 25,
        m: 7,
        s: 7,
        t: 15,
        a: 0x8ebfd028,
        b: 0x2b5b2500,
        c: 0xdb8b0000,
    }
}

func (tg *TGFSR) Initialize(seed uint32) {
    for i := 0; i < tg.n; i++ {
        seed &= 0xffffffff
        tg.x[i] = seed
        seed = (seed*1313 + 88897) & 0xffffffff
    }
    tg.k = tg.n - 1
    tg.initialized = true
}

func (tg *TGFSR) iterate() {
    for i := 0; i < tg.n-tg.m; i++ {
        tg.x[i] = tg.x[i+tg.m] ^ (tg.x[i] >> 1) ^ (func() uint32 {
            if tg.x[i]&1 != 0 {
                return tg.a
            }
            return 0
        }())
    }
    for i := tg.n - tg.m; i < tg.n; i++ {
        tg.x[i] = tg.x[i+tg.m-tg.n] ^ (tg.x[i] >> 1) ^ (func() uint32 {
            if tg.x[i]&1 != 0 {
                return tg.a
            }
            return 0
        }())
    }
}

func (tg *TGFSR) Next() uint32 {
    if !tg.initialized {
        tg.Initialize(1)
    }
    tg.k++
    if tg.k == tg.n {
        tg.iterate()
        tg.k = 0
    }
    y := tg.x[tg.k] ^ ((tg.x[tg.k] << tg.s) & tg.b)
    y ^= (y << tg.t) & tg.c
    return y
}

func (tg *TGFSR) Random() int32 {
    return int32(tg.Next() & 0x7fffffff)
}