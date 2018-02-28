# coding: utf-8
from __future__ import absolute_import

import io

from ._zbox import Repo
from ._zbox import File

io.RawIOBase.register(File)
