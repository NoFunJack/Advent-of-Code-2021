import unittest

from lib.image import Image


class ImageTests(unittest.TestCase):

    def create00(self,algo):
        i = Image("#",algo);
        self.assertEqual(i.map,[(0,0)])
        return i

    def test_delete_everything(self):
        i = self.create00([])
        i.enhance()
        self.assertEqual(i.map,[])

    def test_move_up(self):
        i = self.create00([int('000_000_010',2)])
        i.enhance()
        self.assertEqual(i.map,[(0,-1)])

    def test_multiply(self):
        i = self.create00(
                [int('000_000_001',2),
                int('000_000_010',2),
                int('000_000_100',2)]
                )
        i.enhance()
        self.assertCountEqual(i.map,[(1,-1),(0,-1),(-1,-1)])
        i.enhance()
        self.assertCountEqual(i.map,[(2,-2),(-2,-2)])
