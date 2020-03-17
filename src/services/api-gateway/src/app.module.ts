import { Module } from '@nestjs/common';
import { ClientsModule } from '@nestjs/microservices';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { databaseMicroserviceOptions } from './grpc.options';

@Module({
  imports: [
    ClientsModule.register([
      {
        name: 'DATABASE_PACKAGE',
        ...databaseMicroserviceOptions,
      },
    ]),
  ],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
