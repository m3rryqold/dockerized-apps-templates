# Dockerized Apps Templates
## Pre-requisites
### Docker
https://docs.docker.com/engine/install/ubuntu/#installation-methods

The images in this repo are deployed to docker hub. 
You can clone the repo, make changes (if needed), rebuild the images and use (push) if required.
` PS: all apps in 'm3rrygold' docker hub profile follow the format '${technology}-basic-app' eg. 'flask-basic-app'`

## General steps:
To pull:
```bash
docker pull m3rryqold/<app-name>
```
To run:
```bash
docker run m3rryqold/<app-name>
```
To run containers on another port(-p), and in the background (-d):
```bash
docker run -p <destination-port>:<host-port> -d m3rryqold/<app-name>
```
To check out other arguments for running containers:
```bash
docker run --help
```
Check running containers:
```bash
docker container ls
```