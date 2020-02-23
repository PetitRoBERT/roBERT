import { Logger } from '@nestjs/common';
import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';
import { grpcClientOptions } from './grpc-client.options';

const logger = new Logger('Database');


async function bootstrap() {
  const app = await NestFactory.createMicroservice(AppModule, grpcClientOptions);
  await app.listenAsync();
  logger.log('Database microservice is listening ... ');
}
bootstrap();
