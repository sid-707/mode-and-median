import unittest
from mode import get_mode


class TestMode(unittest.TestCase):
    def test_get_mode_returns_mode_of_list(self):
        list = [1, 2, 3, 3, 4, 5, 6]
        self.assertEqual(get_mode(list), 3)


if __name__ == '__main__':
    unittest.main()
