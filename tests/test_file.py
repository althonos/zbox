# coding: utf-8
from __future__ import absolute_import

import io
import os
import shutil
import tempfile
import unittest
import uuid

from bindings.rust import zbox

class TestFile(unittest.TestCase):

    @classmethod
    def setUpClass(cls):
        cls.tempdir = uuid.uuid4().hex
        cls.repo = zbox.Repo(
            "mem://{}".format(cls.tempdir),
            "pasw",
            create=True
        )

    def setUp(self):
        self.path = "/{}".format(uuid.uuid4().hex)


    def test_iter(self):
        msg = b'abc\ndef\nghi\njkl'
        s = io.BytesIO(msg)

        f = self.repo.open(self.path, 'w')
        f.write(msg)
        f.close()

        for (actual, expected) in zip(f, s):
            self.assertEqual(actual, expected)

    def test_readline(self):
        msg = b'abc\ndef\nghi\njkl'
        s = io.BytesIO(msg)

        f = self.repo.open(self.path, 'w')
        f.write(msg)
        f.close()

        f = self.repo.open(self.path, 'r')

        for line in s:
            self.assertEqual(f.readline(), line)
        self.assertEqual(f.readline(), b'')

        f.close()

    def test_readlines(self):

        msg = b'abc\ndef\nghi\njkl'

        f = self.repo.open(self.path, 'w')
        f.write(msg)
        f.close()

        f = self.repo.open(self.path, 'r')
        self.assertEqual(f.read(), msg)
        f.close()

        f = self.repo.open(self.path, 'r')
        self.assertEqual(f.readlines(), [b'abc\n', b'def\n', b'ghi\n', b'jkl'])
        f.close()

        f = self.repo.open(self.path, 'r')
        self.assertEqual(f.readlines(3), [b'abc\n'])
        f.close()

        f = self.repo.open(self.path, 'r')
        self.assertEqual(f.readlines(5), [b'abc\n', b'def\n'])
        f.close()
