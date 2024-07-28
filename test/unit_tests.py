import unittest

import pyo3_sample
import pyo3_sample.module_a as moda

class TestSampleModule(unittest.TestCase):
    # testing core function access
    def test_root_level(self):
        self.assertEqual(pyo3_sample.sum_as_string(5, 10), '15')

    # testing imported sub-module access
    def test_module_a(self):
        self.assertEqual(moda.add(5, 10), 15)

    # testing non-imported sub-module access
    def test_module_b(self):
        self.assertEqual(pyo3_sample.module_b.sort([5, 1, 4, 2, 3]), [1, 2, 3, 4, 5])

# run all tests
if __name__ == '__main__':
    unittest.main()
