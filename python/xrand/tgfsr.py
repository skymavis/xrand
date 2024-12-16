class TGFSR:
    def __init__(self, seed=1):
        # Constants
        self.N = 25
        self.m = 7
        self.s = 7
        self.t = 15
        self.a = 0x8EBFD028
        self.b = 0x2B5B2500
        self.c = 0xDB8B0000

        # State variables
        self.x = [0] * self.N
        self.k = 0
        self.initialized = False
        
        # Initialize with seed
        self.initialize(seed)

    def initialize(self, seed):
        for i in range(self.N):
            seed &= 0xFFFFFFFF
            self.x[i] = seed
            seed = (seed * 1313 + 88897) & 0xFFFFFFFF
        self.k = self.N - 1
        self.initialized = True

    def iterate(self):
        for i in range(self.N - self.m):
            self.x[i] = self.x[i + self.m] ^ (self.x[i] >> 1) ^ (self.a if self.x[i] & 1 else 0)
        for i in range(self.N - self.m, self.N):
            self.x[i] = self.x[i + self.m - self.N] ^ (self.x[i] >> 1) ^ (self.a if self.x[i] & 1 else 0)

    def next(self):
        if not self.initialized:
            self.initialize(1)
        self.k += 1
        if self.k == self.N:
            self.iterate()
            self.k = 0
        y = self.x[self.k] ^ ((self.x[self.k] << self.s) & self.b)
        y ^= (y << self.t) & self.c
        return y & 0xFFFFFFFF

    def random(self):
        return self.next() & 0x7FFFFFFF