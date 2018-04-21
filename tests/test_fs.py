# coding: utf-8

import io
import os
import time
import unittest
import uuid

import fs
import six

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
        return self.zbfs.makedir(six.text_type(uuid.uuid4().hex))

    # FIXME(@althonos): wait for zboxfs/zbox#27
    @unittest.expectedFailure
    def test_open_files(self):
        super(TestMemZboxFS, self).test_open_files()


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
