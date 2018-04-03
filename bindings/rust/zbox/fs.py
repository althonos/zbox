# coding: utf-8
from __future__ import absolute_import
from __future__ import unicode_literals

import fs.base
import fs.errors
from fs.info import Info
from fs.mode import Mode

from ._zbox import ZboxFS


class ZboxFS(ZboxFS, fs.base.FS):

    _meta = {
        'case_insensitive': False,
        'network': False,
        'read_only': False,
        'invalid_path_chars': '\0',
        'virtual': False,
        'unicode_paths': True,
        'thread_safe': True,
        'virtual': False,
        'supports_rename': True,
    }

    def __init__(self, uri, pwd="", create=False):
        super(ZboxFS, self).__init__()

    def exists(self, path):
        _path = self.validatepath(path)
        return super(ZboxFS, self).exists(_path)

    def getinfo(self, path, namespaces=["basic"]):
        _path = self.validatepath(path)
        rawinfo = super(ZboxFS, self).getinfo(_path, namespaces)
        return Info(rawinfo)

    def isdir(self, path):
        _path = self.validatepath(path)
        return super(ZboxFS, self).isdir(_path)

    def isfile(self, path):
        _path = self.validatepath(path)
        return super(ZboxFS, self).isfile(_path)

    def listdir(self, path):
        _path = self.validatepath(path)
        return super(ZboxFS, self).listdir(_path)

    def makedir(self, path, permissions=None, recreate=False):
        _path = self.validatepath(path)
        super(ZboxFS, self).makedir(_path, recreate=recreate)
        return self.opendir(_path)

    def openbin(self, path, mode="r", buffering=-1, **options):
        _path = self.validatepath(path)
        _mode = Mode(mode).to_platform_bin()
        return super(ZboxFS, self).openbin(_path, _mode, buffering)

    def remove(self, path):
        _path = self.validatepath(path)
        return super(ZboxFS, self).remove(_path)

    def removedir(self, path):
        _path = self.validatepath(path)
        return super(ZboxFS, self).removedir(_path)

    def setinfo(self, path, info):
        _path = self.validatepath(path)
        return super(ZboxFS, self).setinfo(_path, info)
