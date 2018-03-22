# coding: utf-8
from __future__ import absolute_import

import io

from ._zbox import Repo
from ._zbox import File

__all__ = ["File", "Repo"]

try:
    from .fs import ZboxFS
    __all__.append("ZboxFS")
except ImportError:
    pass

io.RawIOBase.register(File)
