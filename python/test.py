import unittest
from xrand import TGFSR

class TestTGFSR(unittest.TestCase):
    def setUp(self):
        self.tgfsr = TGFSR(12345)

    def test_initialization(self):
        """Test if TGFSR initializes properly with a given seed"""
        self.assertIsNotNone(self.tgfsr)
        self.assertTrue(self.tgfsr.initialized)

    def test_random_number_range(self):
        """Test if generated random number is within expected range"""
        random_number = self.tgfsr.random()
        self.assertGreaterEqual(random_number, 0)
        self.assertLessEqual(random_number, 0x7fffffff)

    def test_consecutive_numbers_different(self):
        """Test if consecutive random numbers are different"""
        first_random = self.tgfsr.random()
        second_random = self.tgfsr.random()
        self.assertNotEqual(first_random, second_random)

    def test_seed_consistency(self):
        """Test if same seed produces same sequence"""
        tgfsr1 = TGFSR(12345)
        tgfsr2 = TGFSR(12345)

        random_numbers1 = [tgfsr1.random() for _ in range(10)]
        random_numbers2 = [tgfsr2.random() for _ in range(10)]
        
        self.assertEqual(random_numbers1, random_numbers2)

    def test_multiple_iterations(self):
        """Test if multiple iterations work correctly"""
        numbers = [self.tgfsr.random() for _ in range(100)]
        self.assertEqual(len(numbers), 100)
        # Verify all numbers are unique (or at least most of them)
        unique_numbers = len(set(numbers))
        self.assertGreater(unique_numbers, 90)  # Allow for some potential duplicates

if __name__ == '__main__':
    unittest.main()