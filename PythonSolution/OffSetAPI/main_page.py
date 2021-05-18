root_html_body = """
<body>
<form action="/upload_to_db/" enctype="multipart/form-data" method="post">
    <input name="file" type="file" multiple>
    <input type="submit">
</form>
<form action="/download_subtitle_by_id/" enctype="multipart/form-data" method="get">
    <input type="number" name="id">
    <input type="submit">
</form>
</body>
"""
