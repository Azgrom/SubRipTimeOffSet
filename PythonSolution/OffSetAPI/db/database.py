#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Tue May 18 10:57:45 2021

@author: Rafael Lúcio
"""

from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

SQLALCHEMY_DATABASE_URL = 'sqlite:///./db.sqlite3'

Engine = create_engine(SQLALCHEMY_DATABASE_URL,
                       connect_args = {'check_same_thread': False})
LocalSession = sessionmaker(autocommit = False, autoflush = False,
                              bind = Engine)
Base = declarative_base()
