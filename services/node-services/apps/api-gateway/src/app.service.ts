import { Injectable, OnModuleInit, Inject } from '@nestjs/common';
import { ClientGrpc } from '@nestjs/microservices';
import { AuthorService, Author } from '@app/ts-interfaces';
import { Observable } from 'rxjs';

@Injectable()
export class AppService implements OnModuleInit {
  private authorService: AuthorService;

  constructor(@Inject('DATABASE_PACKAGE') private readonly databaseClient: ClientGrpc) {}

  onModuleInit() {
    this.authorService = this.databaseClient.getService<AuthorService>('AuthorService');
  }

  getAuthor(): Observable<Author> {
    return this.authorService.findOneById({ id: 42 });
  }
}
