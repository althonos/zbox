# coding: utf-8
from __future__ import absolute_import
from __future__ import unicode_literals

import os
import unittest
import uuid

import fs
import six

from fs.test import FSTestCases
from zbox import ZboxFS


class TestMemZboxFS(FSTestCases, unittest.TestCase):

    @classmethod
    def setUpClass(cls):
        cls.zbfs = ZboxFS("mem://")

    @classmethod
    def tearDownClass(cls):
        cls.zbfs.close()

    def make_fs(self):
        return self.zbfs.makedir(six.text_type(uuid.uuid4().hex))


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
