import unittest

from lib.image import Image

class ImageTests(unittest.TestCase):
    def test_test(self):
        i = Image("#",None)
        self.assertEqual(i.mapstr,"#")
