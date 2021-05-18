#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Tue May 18 10:58:49 2021

@author: Rafael Lúcio
"""

import uvicorn as uv
from OffSetAPI.settings import SubOffSetAPI


# '$ python main.py'
if __name__ == '__main__':
    uv.run("main:SubOffSetAPI", host = 'localhost', port = 5000,
           log_level = 'info', reload = True, factory = True, access_log = True)
# '$ uvicorn main:app --reload --factory'
else:
    app = SubOffSetAPI
