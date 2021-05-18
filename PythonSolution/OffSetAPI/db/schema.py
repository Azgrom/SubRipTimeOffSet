#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Tue May 18 11:37:52 2021

@author: Rafael Lúcio
"""

from datetime import datetime
from pydantic import BaseModel, constr, conint


class SubTitleFile(BaseModel):
    file_name: constr(min_length = 1)
    description: constr(max_length = 100)

class Subtitle(BaseModel):
    sub_nmb: conint()
    start_time: datetime
    end_time: datetime
    dialog: str
