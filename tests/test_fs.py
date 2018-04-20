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
        return self.zbfs.makedir(uuid.uuid4().hex)

    def test_open_files(self):
        # Test file-like objects work as expected.

        with self.fs.open('text', 'w') as f:
            repr(f)
            six.text_type(f)
            self.assertIsInstance(f, io.IOBase)
            self.assertTrue(f.writable())
            self.assertFalse(f.readable())
            self.assertFalse(f.closed)
            self.assertEqual(f.tell(), 0)
            f.write('Hello\nWorld\n')
            self.assertEqual(f.tell(), 12)
            f.writelines(['foo\n', 'bar\n', 'baz\n'])
            with self.assertRaises(IOError):
                f.read(1)
        self.assertTrue(f.closed)

        with self.fs.open('bin', 'wb') as f:
            with self.assertRaises(IOError):
                f.read(1)

        with self.fs.open('text', 'r') as f:
            repr(f)
            six.text_type(f)
            self.assertIsInstance(f, io.IOBase)
            self.assertFalse(f.writable())
            self.assertTrue(f.readable())
            self.assertFalse(f.closed)
            self.assertEqual(
                f.readlines(),
                ['Hello\n', 'World\n', 'foo\n', 'bar\n', 'baz\n']
            )
            with self.assertRaises(IOError):
                f.write('no')
        self.assertTrue(f.closed)

        with self.fs.open('text', 'rb') as f:
            self.assertIsInstance(f, io.IOBase)
            self.assertFalse(f.writable())
            self.assertTrue(f.readable())
            self.assertFalse(f.closed)
            self.assertEqual(
                f.readlines(8),
                [b'Hello\n', b'World\n']
            )
            with self.assertRaises(IOError):
                f.write(b'no')
        self.assertTrue(f.closed)

        with self.fs.open('text', 'r') as f:
            self.assertEqual(
                list(f),
                ['Hello\n', 'World\n', 'foo\n', 'bar\n', 'baz\n']
            )
            self.assertFalse(f.closed)
        self.assertTrue(f.closed)

        iter_lines = iter(self.fs.open('text'))
        self.assertEqual(next(iter_lines), 'Hello\n')

        with self.fs.open('unicode', 'w') as f:
            self.assertEqual(12, f.write('Héllo\nWörld\n'))

        with self.fs.open('text', 'rb') as f:
            self.assertIsInstance(f, io.IOBase)
            self.assertFalse(f.writable())
            self.assertTrue(f.readable())
            self.assertTrue(f.seekable())
            self.assertFalse(f.closed)
            self.assertEqual(f.read(1), b'H')
            self.assertEqual(3, f.seek(3, fs.Seek.set))
            self.assertEqual(f.read(1), b'l')
            self.assertEqual(6, f.seek(2, fs.Seek.current))
            self.assertEqual(f.read(1), b'W')
            self.assertEqual(22, f.seek(-2, fs.Seek.end))
            self.assertEqual(f.read(1), b'z')
            with self.assertRaises(ValueError):
                f.seek(10, 77)
        self.assertTrue(f.closed)

        with self.fs.open('text', 'r+b') as f:
            self.assertIsInstance(f, io.IOBase)
            self.assertTrue(f.readable())
            self.assertTrue(f.writable())
            self.assertTrue(f.seekable())
            self.assertFalse(f.closed)
            self.assertEqual(5, f.seek(5))
            self.assertEqual(5, f.truncate())
            self.assertEqual(0, f.seek(0))
            self.assertEqual(f.read(), b'Hello')
            self.assertEqual(10, f.truncate(10))
            self.assertEqual(5, f.tell())
            self.assertEqual(0, f.seek(0))
            print(repr(self.fs))
            print(repr(f))
            self.assertEqual(f.read(), b'Hello\0\0\0\0\0')
            self.assertEqual(4, f.seek(4))
            f.write(b'O')
            self.assertEqual(4, f.seek(4))
            self.assertEqual(f.read(1), b'O')
        self.assertTrue(f.closed)


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
