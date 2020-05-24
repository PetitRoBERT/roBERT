FROM petitrobert/base-node AS build-node-services

WORKDIR /services/node-services/

COPY package.json ./
COPY yarn.lock ./

RUN yarn install --only=development

COPY . .

RUN yarn build api-gateway
RUN yarn build database

FROM petitrobert/base-node as production-node-services

ARG NODE_ENV=production
ENV NODE_ENV=${NODE_ENV}

WORKDIR /services/node-services/

COPY package.json ./
COPY yarn.lock ./

RUN yarn install --only=production

COPY . .

COPY --from=build-node-services /services/node-services/dist ./dist

WORKDIR /services/node-services/dist/apps/

ENTRYPOINT [ "node" ]
