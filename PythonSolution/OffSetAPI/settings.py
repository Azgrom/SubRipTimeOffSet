#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Tue May 18 10:56:41 2021

@author: Rafael Lúcio
"""

import shutil
from fastapi import Depends, FastAPI, File, HTTPException, UploadFile
from fastapi.responses import HTMLResponse, FileResponse
from pathlib import Path
from sqlalchemy.orm import Session
from tempfile import NamedTemporaryFile

from .db import model, schema
from .db.database import Engine, LocalSession
from .db.session import PostMethods, GetMethods
from .file_processing.offset_method import offset_sub
from .file_processing.uploaded_files import save_upload_file_tmp
from .main_page import root_html_body


class SubOffSetAPI(FastAPI):
    model.Base.metadata.create_all(bind = Engine)

    subs_rs = ['Subtitle History']
    temp_file_str = '/tmp/tmpe6hf36lu.srt'

    def __init__(self, title: str = 'Subtitle Time OffSet API') -> None:
        super().__init__()

        def get_db():
            db = LocalSession()
            try:
                yield db
            finally:
                db.close()

        @self.post("/upload_subtitle/", tags = self.subs_rs)
        async def upload_temp_subtitle(file: UploadFile = File(...)):
            return save_upload_file_tmp(file)

        @self.post("/upload_to_db/")
        async def upload_to_db(db: Session = Depends(get_db),
                               file: UploadFile = File(...)):
            check_db = GetMethods(db).list_db_by_title(title = file.filename)
            if check_db:
                raise HTTPException(status_code = 401,
                                    detail = "File already exists in database")

            sub = schema.SubTitleSchema(file_name = file.filename,
                                        file_content = file.file.read())
            dub_db = PostMethods(db).register_subtitle(sub)
            return file

        @self.get("/download_subtitle/")
        async def download_subtitle():
            return FileResponse(self.temp_file_str)

        @self.get("/")
        async def main():
            content = root_html_body
            return HTMLResponse(content=content)
