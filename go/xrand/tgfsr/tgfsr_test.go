package tgfsr

import (
	"testing"
)

func TestTGFSR(t *testing.T) {
	t.Run("should initialize with a given seed", func(t *testing.T) {
		tgfsr := New()
		tgfsr.Initialize(12345)
		if !tgfsr.initialized {
			t.Error("TGFSR should be initialized, but it was not")
		}
	})

	t.Run("should generate a random number within bounds", func(t *testing.T) {
		tgfsr := New()
		tgfsr.Initialize(12345)
		
		randomNumber := tgfsr.Random()
		if randomNumber < 0 || randomNumber > 0x7fffffff {
			t.Errorf("Random number %d should be between 0 and 0x7fffffff, but it was not", randomNumber)
		}
	})

	t.Run("should generate different random numbers on consecutive calls", func(t *testing.T) {
		tgfsr := New()
		tgfsr.Initialize(12345)
		
		firstRandom := tgfsr.Random()
		secondRandom := tgfsr.Random()
		if firstRandom == secondRandom {
			t.Error("Consecutive random numbers should be different, but they were the same")
		}
	})

	t.Run("should produce consistent results for the same seed", func(t *testing.T) {
		tgfsr1 := New()
		tgfsr1.Initialize(12345)
		tgfsr2 := New()
		tgfsr2.Initialize(12345)

		randomNumbers1 := make([]int32, 10)
		randomNumbers2 := make([]int32, 10)

		for i := 0; i < 10; i++ {
			randomNumbers1[i] = tgfsr1.Random()
			randomNumbers2[i] = tgfsr2.Random()
		}

		for i := 0; i < 10; i++ {
			if randomNumbers1[i] != randomNumbers2[i] {
				t.Errorf("Random sequences with same seed should be identical at position %d: %d != %d",
					i, randomNumbers1[i], randomNumbers2[i])
			}
		}
	})

	t.Run("should handle multiple iterations correctly", func(t *testing.T) {
		tgfsr := New()
		tgfsr.Initialize(12345)

		numbers := make([]int32, 100)
		for i := 0; i < 100; i++ {
			numbers[i] = tgfsr.Random()
		}

		// Verify we can generate 100 numbers without panicking
		if len(numbers) != 100 {
			t.Error("Should generate 100 numbers, but did not")
		}
	})
}