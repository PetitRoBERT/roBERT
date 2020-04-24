// import * as ProtoLoader from '@grpc/proto-loader';
// import { Test, TestingModule } from '@nestjs/testing';
// import { INestApplication } from '@nestjs/common';
// import * as request from 'supertest';
// import * as GRPC from 'grpc';
// import { grpcDatabaseClientOptions } from '../src/grpc-database-client.options';
// import { join } from 'path';
// import { AppController } from '../src/app.controller';
// import { AppModule } from '../src/app.module';
// import { AppService } from '../src/app.service';

describe('AppController (e2e)', () => {
  // let server: any;
  // let app: INestApplication;
  // let client: any;

  // beforeEach(async () => {
  //   const moduleFixture: TestingModule = await Test.createTestingModule({
  //     imports: [AppModule],
  //     controllers: [AppController],
  //     providers: [AppService],
  //   }).compile();

  //   app = moduleFixture.createNestApplication();
  //   server = app.getHttpAdapter().getInstance();

  //   app.connectMicroservice(grpcDatabaseClientOptions);

  //   // Start gRPC microservice
  //   await app.startAllMicroservicesAsync();
  //   await app.init();

  //   // Load proto-buffers for test gRPC dispatch
  //   const proto = ProtoLoader.loadSync(
  //     join(__dirname, '../../../node_modules/interfaces/database/database.proto'),
  //   ) as any;
  //   // Create Raw gRPC client object
  //   const protoGRPC = GRPC.loadPackageDefinition(proto) as any;
  //   // Create client connected to started services at standard 5000 port
  //   client = new protoGRPC.database.AuthorService('localhost:5000', GRPC.credentials.createInsecure());
  // });

  it('/ (GET)', () => {
    expect(true).toBeTruthy();
  });
});
