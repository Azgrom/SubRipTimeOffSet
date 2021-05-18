#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Tue May 18 11:03:23 2021

@author: Rafael Lúcio
"""

from sqlalchemy import Column, Integer, String, Text

from .database import Base


class Subtitles(Base):
    __tablename__ = "subtitle_submissions"

    id = Column(Integer, primary_key = True, index = True)
    file_name = Column(String, unique = True)
    file_content = Column(Text)
