FROM node:16-alpine
WORKDIR /usr/src/app

COPY package*.json .
RUN npm ci --production

COPY src/ src/

USER node
ENV NODE_ENV=production

CMD [ "node", "." ]
