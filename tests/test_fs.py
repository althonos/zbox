# coding: utf-8

import time
import unittest
import uuid

from fs.test import FSTestCases
from bindings.rust.zbox import ZboxFS


@unittest.skip("Segfaults !")
class TestZboxFS(FSTestCases, unittest.TestCase):

    def make_fs(self):
        fs = ZboxFS("mem://")
        fs.removetree('/')
        return fs

    def destroy_fs(self, fs):
        super(TestZboxFS, self).destroy_fs(fs)
        time.sleep(2)
