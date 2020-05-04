import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { ClientsModule } from '@nestjs/microservices';
import { grpcDatabaseClientOptions } from './grpc-database-client.options';
import { AuthorModule } from './author/author.module';

@Module({
  imports: [
    ClientsModule.register([
      {
        name: 'DATABASE_PACKAGE',
        ...grpcDatabaseClientOptions,
      },
    ]),
    AuthorModule,
  ],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
