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

  def test_nothing_is_something(self):
      i = self.create00([
          0,
          int('111_000_111',2),
          int('000_010_000',2),
          ])
      i.enhance()
      self.assertTrue(i.out_bounds_state)
      self.assertEqual(len(i.map),1)
      i.enhance()
      self.assertEqual(len(i.map),1)

  def test_nothing_is_and_more(self):
      i = self.create00([
          0,
          int('111_000_010',2),
          int('000_010_000',2),
          ])
      print(i)
      i.enhance()
      print(i)
      self.assertTrue(i.out_bounds_state)
      self.assertEqual(len(i.map),1)
      i.enhance()
      print(i)
      self.assertEqual(len(i.map),2)

  def test_always_new_stuff_incorner(self):
      i = self.create00([
          0,
          int('111_100_100',2),
          int('111_100_101',2),
          int('000_010_000',2),
          ])
      i.enhance()
      self.assertTrue(i.out_bounds_state)
      self.assertEqual(len(i.map),1)
      i.enhance()
      self.assertEqual(len(i.map),2)
      i.enhance()
      self.assertEqual(len(i.map),2)

  def test_dent(self):
      i = Image(
"""
#...#
#####
"""
          ,[
          0,
          int('000_000_111',2),
          ])
      i.enhance()
      self.assertTrue(i.out_bounds_state)
      self.assertEqual(len(i.map),2)
      i.enhance()
      self.assertEqual(len(i.map),0)

  def test_blink_blank(self):
      i = Image('',[i for i in range(0,511)])
      self.assertFalse(int('111_111_111',2) in i.algoArr) 
      i.enhance()
      self.assertTrue(i.out_bounds_state)
      i.enhance()
      self.assertEqual(len(i.map),0)
