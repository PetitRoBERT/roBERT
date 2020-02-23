import { Transport, ClientOptions } from "@nestjs/microservices";
import { join } from "path";

export const grpcClientOptions: ClientOptions = {
    transport: Transport.GRPC,
    options: {
      package: 'database',
      protoPath: join(__dirname, '../src/database.proto'),
    },
  };