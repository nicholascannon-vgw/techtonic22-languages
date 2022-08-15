FROM node:16-alpine
WORKDIR /usr/src/app

# Install prod dependencies
COPY package*.json .
RUN npm ci --production

# Copy app code
COPY src/ src/

# Use non-root user
USER node

ENV NODE_ENV=production

CMD [ "node", "." ]
