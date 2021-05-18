from sqlalchemy.orm import Session
from typing import Optional

from . import model, schema


class InitializeSession:
    def __init__(self, db: Session, skip: Optional[int] = 0,
                 limit: Optional[int] = 100):
        self.db = db
        self.skip = skip
        self.limit = limit


class GetMethods(InitializeSession):
    def get_title_occurrence(self, title: str):
        field = model.Subtitles.file_name
        title_filter = db_field == title
        title_occurence = self.db.query(model.Subtitles).filter(title_filter)
        return title_occurence.offset(self.skip).limit(self.limit).all()

    def list_filenames(self):
        field = model.Subtitles.file_name
        return self.db.query(field).offset(self.skip).limit(self.limit).all()

    def get_sub_data(self, id: int):
        id_db_data = self.db.query(model.Subtitles)
        return id_db_data.get(id)


class PostMethods(InitializeSession):
    def register_subtitle(self, sub_schema: schema.SubTitleSchema):
        db_subt = model.Subtitles(**sub_schema.dict())
        self.db.add(db_subt)
        self.db.commit()
        self.db.refresh(db_subt)
        return db_subt
