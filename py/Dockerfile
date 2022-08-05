FROM python:3.9-alpine
WORKDIR /usr/src/app

RUN pip install gunicorn

COPY requirements.txt requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

COPY src/ src/

CMD [ "gunicorn", "-b", "0.0.0.0:8000", "src.app:app" ]
