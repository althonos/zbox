# coding: utf-8
from __future__ import absolute_import
from __future__ import unicode_literals

import fs.base
import fs.errors
import fs.mode
from fs.info import Info

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

    def copy(self, src, dst, overwrite=False):
        _src = self.validatepath(src)
        _dst = self.validatepath(dst)
        return super(ZboxFS, self).copy(_src, _dst, overwrite)

    def exists(self, path):
        _path = self.validatepath(path)
        return super(ZboxFS, self).exists(_path)

    def getinfo(self, path, namespaces=["basic"]):
        _path = self.validatepath(path)
        rawinfo = super(ZboxFS, self).getinfo(_path, namespaces)
        return Info(rawinfo)

    # def getfile(self, path, file, chunk_size=None, **options):
    #     # type: (Text, BinaryIO, Optional[int], **Any) -> None
    #     with self._lock:
    #         with self.openbin(path, **options) as read_file:
    #             tools.copy_file_data(
    #                 read_file,
    #                 file,
    #                 chunk_size=chunk_size
    #             )

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

    def move(self, src, dst, overwrite=False):
        _src = self.validatepath(src)
        _dst = self.validatepath(dst)
        return super(ZboxFS, self).move_(_src, _dst, overwrite)

    def openbin(self, path, mode="r", buffering=-1, **options):
        _path = self.validatepath(path)
        fs.mode.validate_openbin_mode(mode)
        return super(ZboxFS, self).openbin(_path, mode, buffering)

    def remove(self, path):
        _path = self.validatepath(path)
        return super(ZboxFS, self).remove(_path)

    def removedir(self, path):
        _path = self.validatepath(path)
        return super(ZboxFS, self).removedir(_path)

    def setinfo(self, path, info):
        _path = self.validatepath(path)
        return super(ZboxFS, self).setinfo(_path, info)
