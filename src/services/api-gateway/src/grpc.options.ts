import { ClientOptions, Transport } from '@nestjs/microservices';
import { join } from 'path';

export const databaseMicroserviceOptions: ClientOptions = {
  transport: Transport.GRPC,
  options: {
    package: 'database',
    protoPath: join(__dirname, '../../../interfaces/database/database.proto'),
  },
};
