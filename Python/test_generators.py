import unittest
from generators import generate_sorted_list


class TestGenerators(unittest.TestCase):
    def test_generate_sorted_list_with_length(self):
        length = 3
        list = generate_sorted_list(length)
        self.assertEqual(len(list), length)


if __name__ == '__main__':
    unittest.main()
