# coding: utf-8
from __future__ import absolute_import

import io
import os
import shutil
import tempfile
import unittest
import uuid

import zbox

class _TestRepo(object):

    def test_create_dir(self):
        self.assertEqual(self.repo.read_dir('/'), [])
        self.assertFalse(self.repo.path_exists('/test'))
        self.assertFalse(self.repo.is_dir('/test'))
        self.assertFalse(self.repo.is_file('/test'))
        self.repo.create_dir('/test')
        self.assertTrue(self.repo.read_dir('/'))
        self.assertEqual(self.repo.read_dir('/')[0]['file_name'], 'test')
        self.assertTrue(self.repo.path_exists('/test'))
        self.assertTrue(self.repo.is_dir('/test'))
        self.assertFalse(self.repo.is_file('/test'))

    def test_create_dir_all(self):
        self.assertEqual(self.repo.read_dir('/'), [])
        self.repo.create_dir_all('/foo/bar/baz')
        self.assertTrue(self.repo.is_dir('/foo'))
        self.assertTrue(self.repo.is_dir('/foo/bar'))
        self.assertTrue(self.repo.is_dir('/foo/bar/baz'))

    # def test_read_dir(self):
        # self.assertEqual(self.repo.read_dir('/'), [])
        # self.repo.create_dir('/foo')
        # self.repo.create_dir_all('/bacon/spam/eggs')

    def test_open(self):

        f = self.repo.open('/test.txt', mode='w')
        self.assertEqual(f.write(b'test'), 4)
        f.truncate(5)
        f.close()

        self.assertRaises(ValueError, f.write, b'test',)

        f = self.repo.open('/test.txt', mode='r')
        self.assertEqual(f.read(), b'test\0')

        self.assertIsInstance(f, io.RawIOBase)
        self.assertIsInstance(f, io.IOBase)



class TestDirectoryRepo(_TestRepo, unittest.TestCase):

    def setUp(self):
        self.tempdir = tempfile.mkdtemp()
        os.rmdir(self.tempdir)
        self.repo = zbox.Repo(
            "file://{}".format(self.tempdir),
            "pasw",
            create=True,
        )

    def tearDown(self):
        del self.repo
        shutil.rmtree(self.tempdir)


class TestMemoryRepo(_TestRepo, unittest.TestCase):

    def setUp(self):
        self.tempdir = uuid.uuid4().hex
        self.repo = zbox.Repo(
            "mem://{}".format(self.tempdir),
            "pasw",
            create=True
        )

    def tearDown(self):
        del self.repo
