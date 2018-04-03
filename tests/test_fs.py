# coding: utf-8

import time
import unittest
import uuid

import fs

from fs.test import FSTestCases
from bindings.rust.zbox import ZboxFS


# @unittest.skip("Segfaults !")
class TestZboxFS(FSTestCases, unittest.TestCase):

    @classmethod
    def setUpClass(cls):
        cls.zbfs = ZboxFS("mem://")

    @classmethod
    def tearDownClass(cls):
        cls.zbfs.close()

    def make_fs(self):
        if self.zbfs.isclosed():
            self.zbfs = ZboxFS("mem://")
        return self.zbfs

    def destroy_fs(self, zbfs):
        if not zbfs.isclosed():
            zbfs.removetree("/")

    @unittest.skip("avoid closing the filesystem")
    def test_close(self):
        pass
