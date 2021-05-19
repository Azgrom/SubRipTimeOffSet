#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Tue May 18 10:56:41 2021

@author: Rafael Lúcio
"""

import shutil
from fastapi import Depends, FastAPI, File, Query, HTTPException, UploadFile
from fastapi.responses import HTMLResponse, FileResponse, RedirectResponse
from pathlib import Path
from sqlalchemy.orm import Session
from tempfile import NamedTemporaryFile

from .db import model, schema
from .db.database import Engine, LocalSession
from .db.session import PostMethods, GetMethods
from .file_processing.offset_method import offset_sub
from .file_processing.uploaded_files import (save_upload_file_tmp,
                                            save_db_file_tmp)
from .main_page import root_html_body


class SubOffSetAPI(FastAPI):
    model.Base.metadata.create_all(bind = Engine)

    subs_rs = ['Subtitle History']

    def __init__(self, title: str = 'Subtitle Time OffSet API') -> None:
        super().__init__()

        def get_db():
            db = LocalSession()
            try:
                yield db
            finally:
                db.close()

        @self.post("/upload_to_db/")
        async def upload_to_db(db: Session = Depends(get_db),
                               file: UploadFile = File(...)):
            check_db = GetMethods(db).get_title_occurrence(title = file.filename)
            if check_db:
                raise HTTPException(status_code = 401,
                                    detail = "File already exists in database")

            sub = schema.SubTitleSchema(file_name = file.filename,
                                        file_content = file.file.read())
            dub_db = PostMethods(db).register_subtitle(sub)
            return file

        @self.get("/list_files_in_db/")
        async def list_files(db: Session = Depends(get_db), skip: int = 0,
                             limit: int = 100):
            fields = GetMethods(db, skip, limit).list_filenames()
            if fields == [] or fields == None:
                raise HTTPException(status_code = 403,
                                    detail = "Database is empty.")
            return fields

        @self.get("/download_subtitle_by_id/{id}")
        async def download_subtitle_with_offset(db: Session = Depends(get_db),
                                                id: int = Query(...),
                                                offset: float = Query(...)):
            entry = GetMethods(db).get_sub_data(id)
            if id < 1:
                raise HTTPException(status_code = 402,
                                    detail = "ID must be equal or greater than 1")
            elif entry == [] or entry is None:
                raise HTTPException(status_code = 402,
                                    detail = "There is no entry with this ID")
            created_file_from_db = save_db_file_tmp(entry)
            offset_sub(created_file_from_db, offset)
            return FileResponse(path = created_file_from_db,
                                media_type='application/octet-stream',
                                filename = entry.file_name)

        @self.post("/upload_subtitle/", tags = self.subs_rs, description = 'This route saves the uploaded file to the temporary system directory')
        async def upload_temp_subtitle(file: UploadFile = File(...)):
            return save_upload_file_tmp(file)

        @self.get("/")
        async def main():
            return RedirectResponse('http://localhost:80/docs')
