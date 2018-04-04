# coding: utf-8

import os
import time
import unittest
import uuid

import fs

from fs.test import FSTestCases
from bindings.rust.zbox import ZboxFS


# @unittest.skip("Segfaults !")
class TestMemZboxFS(FSTestCases, unittest.TestCase):

    @classmethod
    def setUpClass(cls):
        cls.zbfs = ZboxFS("mem://")

    @classmethod
    def tearDownClass(cls):
        cls.zbfs.close()

    def make_fs(self):
        return self.zbfs.makedir(uuid.uuid4().hex)


class TestFileZboxFS(TestMemZboxFS):

    @classmethod
    def setUpClass(cls):
        cls.tmp = fs.open_fs("temp://")
        os.rmdir(cls.tmp.getsyspath('/'))
        cls.zbfs = ZboxFS(cls.tmp.geturl('/'))

    @classmethod
    def tearDownClass(cls):
        cls.zbfs.close()
        cls.tmp.close()
