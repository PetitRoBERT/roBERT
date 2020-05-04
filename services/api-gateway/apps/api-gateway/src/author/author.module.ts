import { Module } from '@nestjs/common';
import { AuthorController } from './author.controller';
import { AuthorService } from './author.service';
import { grpcDatabaseClientOptions } from '../grpc-database-client.options';
import { ClientsModule } from '@nestjs/microservices';

@Module({
  imports:[
    ClientsModule.register([
      {
        name: 'DATABASE_PACKAGE',
        ...grpcDatabaseClientOptions,
      },
    ]),
  ],
  controllers: [AuthorController],
  providers: [AuthorService]
})
export class AuthorModule {}
