FROM node:16.15.0-alpine
WORKDIR /usr/src/app

COPY package.json .
COPY package-lock.json .
RUN npm ci --production

COPY src/ src/

ENV NODE_ENV=production

CMD [ "node", "./src/main.js" ]
