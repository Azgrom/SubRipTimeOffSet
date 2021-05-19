#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Tue May 18 09:57:31 2021

@author: Rafael Lúcio
"""

import srt
import datetime as dt

def offset_sub(subtitle_file_path: str, offset_in_secs: float = 0):
    with open(subtitle_file_path, 'r') as f:
        data = f.read()

    subs = list(srt.parse(data))
    for sub in subs:
        sub.start += dt.timedelta(seconds = offset_in_secs)
        sub.end += dt.timedelta(seconds = offset_in_secs)

    with open(subtitle_file_path, 'w') as f:
        f.write(format(srt.compose(subs)))
