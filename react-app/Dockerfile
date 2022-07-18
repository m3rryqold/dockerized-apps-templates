FROM node:16.16 as build

WORKDIR /app

COPY package*.json .
RUN npm install
COPY . .
RUN npm run build