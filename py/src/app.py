from flask import Flask

app = Flask(__name__)

@app.route("/healthcheck")
def healthcheck():
    return { "message": "healthy" }
