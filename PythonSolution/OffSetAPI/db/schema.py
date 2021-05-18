#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Tue May 18 11:37:52 2021

@author: Rafael Lúcio
"""

from datetime import datetime
from pydantic import BaseModel, constr


class SubtitleFileName(BaseModel):
    file_name: constr(min_length = 1)


class SubTitleSchema(SubtitleFileName):
    file_name: constr(min_length = 1)
    file_content: str
