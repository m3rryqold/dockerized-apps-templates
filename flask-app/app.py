from flask import Flask

app = Flask(__name__)

@app.route("/")
def hello():
    return "Welcome to your very own Flask basic application"

if __name__ == "__main__":
    app.run(host="0.0.0.0")
