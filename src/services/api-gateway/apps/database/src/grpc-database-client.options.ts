import { Transport, ClientOptions } from '@nestjs/microservices';
import { join } from 'path';

export const grpcDatabaseClientOptions: ClientOptions = {
  transport: Transport.GRPC,
  options: {
    package: 'database',
    protoPath: join(__dirname, '../../../node_modules/interfaces/database/database.proto'),
  },
};
