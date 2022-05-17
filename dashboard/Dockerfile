FROM node:16 AS builder
ENV NODE_ENV development
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm install
COPY . .
RUN npm run re:build
RUN npm run build

FROM nginx:1.21.0-alpine as production
ENV NODE_ENV production
COPY --from=builder /app/build /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 4000
CMD ["nginx", "-g", "daemon off;"]
