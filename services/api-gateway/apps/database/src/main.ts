import { Logger } from '@nestjs/common';
import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';
import { grpcDatabaseClientOptions } from './grpc-database-client.options';

const logger = new Logger('Database');

async function bootstrap() {
  const app = await NestFactory.createMicroservice(AppModule, grpcDatabaseClientOptions);
  await app.listenAsync();
  logger.log('Database microservice is listening ... ');
}
bootstrap();
