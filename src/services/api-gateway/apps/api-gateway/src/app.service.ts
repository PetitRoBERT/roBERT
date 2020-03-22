import { Injectable, OnModuleInit, Inject } from '@nestjs/common';
import { ClientGrpc } from '@nestjs/microservices';
import { AuthorService } from 'interfaces';

@Injectable()
export class AppService implements OnModuleInit {
  private authorService: AuthorService;

  constructor(@Inject('DATABASE_PACKAGE') private readonly databaseClient: ClientGrpc) {}

  onModuleInit() {
    this.authorService = this.databaseClient.getService<AuthorService>('AuthorService');
  }

  getAuthor() {
    return this.authorService.findOneById({ id: 42 });
  }
}
