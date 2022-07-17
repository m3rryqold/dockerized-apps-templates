## Image deployed on docker hub:

To pull:
```bash
docker pull m3rryqold/flask-basic-app
```
To run:
```bash
docker run m3rryqold/flask-basic-app
```
By default, flask runs on port 5000. To run on another port(-p), and in the background (-d):
```bash
docker run -p 5001:5000 -d m3rryqold/flask-basic-app
```
Check running containers:
```bash
docker container ls
```