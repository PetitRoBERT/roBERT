import { Module } from '@nestjs/common';
import { ClientsModule } from '@nestjs/microservices';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { microserviceOptions } from './grpc.options';

@Module({
  imports: [
    ClientsModule.register([
      {
        name: 'DATABASE_PACKAGE',
        // TODO: rename based on which micro service
        ...microserviceOptions
      }
    ])
  ],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule { }
