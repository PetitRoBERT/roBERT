FROM base-node AS build-api-gateway

WORKDIR /services/api-gateway/

COPY package.json ./
COPY yarn.lock ./

RUN yarn install --only=development

COPY . .

RUN yarn build api-gateway
RUN yarn build database

FROM base-node as production-api-gateway

ARG NODE_ENV=production
ENV NODE_ENV=${NODE_ENV}

WORKDIR /services/api-gateway/

COPY package.json ./
COPY yarn.lock ./

RUN yarn install --only=production

COPY . .

COPY --from=build-api-gateway /services/api-gateway/dist ./dist

WORKDIR /services/api-gateway/dist/apps/

RUN ls

ENTRYPOINT [ "node" ]
