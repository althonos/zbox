# coding: utf-8
from __future__ import absolute_import
from __future__ import unicode_literals

import fs.base

from ._zbox import ZboxFS


class ZboxFS(ZboxFS, fs.base.FS):

    def __init__(self, uri, pwd="", create=False):
        super(ZboxFS, self).__init__()

    def exists(self, path):
        _path = self.validatepath(path)
        return super(ZboxFS, self).exists(_path)

    def isdir(self, path):
        _path = self.validatepath(path)
        return super(ZboxFS, self).isdir(_path)

    def isfile(self, path):
        _path = self.validatepath(path)
        return super(ZboxFS, self).isfile(_path)

    def makedir(self, path):
        _path = self.validatepath(path)
        return super(ZboxFS, self).makedir(_path)

    def openbin(self, path, mode="r", buffering=-1, **options):
        _path = self.validatepath(path)
        return super(ZboxFS, self).openbin(_path)

    def remove(self, path):
        _path = self.validatepath(path)
        return super(ZboxFS, self).remove(_path)

    def removedir(self, path):
        _path = self.validatepath(path)
        return super(ZboxFS, self).removedir(_path)

    def setinfo(self, path, info):
        _path = self.validatepath(path)
        return super(ZboxFS, self).setinfo(_path, info)
