#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Tue May 18 10:56:41 2021

@author: Rafael Lúcio
"""

from fastapi import FastAPI, File, UploadFile

from .db import models
from .db.database import Engine, LocalSession


class SubOffSetAPI(FastAPI):
    models.Base.metadata.create_all(bind = Engine)

    subs_rs = ['Subtitle History']

    def __init__(self, title: str = 'Subtitle Time OffSet API') -> None:
        super().__init__()

        def get_db():
            db = LocalSession()
            try:
                yield db
            finally:
                db.close()

        @self.post("/upload_subtitle/", tags = self.subs_rs)
        async def upload_subtitle(file: UploadFile = File(...)):
            return {'filename': file.filename}
