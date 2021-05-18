#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Tue May 18 11:03:23 2021

@author: Rafael Lúcio
"""

from sqlalchemy import Column, ForeignKey, Integer, String, Time
from sqlalchemy.orm import relationship

from .database import Base

class File(Base):
    __tablename__ = "subtitle_files"

    id = Column(Integer, primary_key = True, index = True)
    file_name = Column(String, unique = True, index = True)
    description = Column(String, index = True)

    item = relationship('Subtitle', back_populates = 'owner')


class Subtitles(Base):
    __tablename__ = "subtitle_contents"

    id = Column(Integer, primary_key = True, index = True)
    sub_nmb = Column(Integer)
    dialog_start_time = Column(Time)
    dialog_end_time = Column(Time)
    owner_id = Column(Integer, ForeignKey('subtitle_files.id'))

    owner = relationship('File', back_populates='subtitle_contents')
