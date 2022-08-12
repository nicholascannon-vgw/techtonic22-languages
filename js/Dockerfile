FROM node:16-alpine
WORKDIR /usr/src/app

# Download app dependencies
COPY package*.json .
RUN npm ci --production

# Copy app code
COPY src/ src/

USER node
ENV NODE_ENV=production

CMD [ "node", "." ]
