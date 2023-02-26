import unittest
from median import get_median


class TestMedian(unittest.TestCase):
    def test_get_median_returns_median_of_list_with_even_number_of_elements(self):
        list = [1, 2, 3, 4, 5, 6]
        self.assertEqual(get_median(list), 3.5)

    def test_get_median_returns_median_of_list_with_odd_number_of_elements(self):
        list = [1, 2, 3, 4, 5]
        self.assertEqual(get_median(list), 3)


if __name__ == '__main__':
    unittest.main()
