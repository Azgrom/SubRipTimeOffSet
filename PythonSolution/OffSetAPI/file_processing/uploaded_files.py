import os
import shutil
from fastapi import UploadFile
from pathlib import Path
from tempfile import NamedTemporaryFile, SpooledTemporaryFile

from OffSetAPI.db.model import Subtitles


def save_upload_file_tmp(upload_file: UploadFile) -> Path:
    try:
        suffix = Path(upload_file.filename).suffix
        with NamedTemporaryFile(delete=False, suffix=suffix) as tmp:
            shutil.copyfileobj(upload_file.file, tmp)
            tmp_path = Path(tmp.name)
    finally:
        upload_file.file.close()
    return tmp_path

def save_db_file_tmp(db_entry: Subtitles) -> str:
    if os.path.exists('cache/'):
        pass
    else:
        os.mkdir('cache/')

    file_path = f'cache/{db_entry.file_name}'
    with open(file_path, mode = 'w') as f:
        f.write(db_entry.file_content)

    return file_path
