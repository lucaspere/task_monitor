FROM node:18.18.2-alpine

WORKDIR /app

COPY package.json .

RUN node --version
RUN npm install

RUN ls
COPY . .
RUN npm run build:ts

EXPOSE 3000

ENV ADDRESS=0.0.0.0 PORT=3000

ENV GOOGLE_APPLICATION_CREDENTIALS=/app/application_default_credentials.json
CMD ["npm", "start"]
