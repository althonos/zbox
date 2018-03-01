# coding: utf-8
from __future__ import absolute_import

import os
import sys

sys.path.append(os.path.abspath(os.path.join(__file__, "..", "..")))
sys.path.append(os.path.abspath(os.path.join(__file__, "..", "..", "build", "lib")))

from . import test_file
from . import test_repo
