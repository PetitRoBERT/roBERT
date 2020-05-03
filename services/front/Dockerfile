FROM petitrobert/base-node AS build-front

WORKDIR /services/front/

COPY package.json ./
COPY yarn.lock ./

RUN yarn install

COPY . .

RUN NODE_ENV=production yarn build

FROM nginx:stable-alpine as production-front

COPY --from=build-front /services/front/build /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
