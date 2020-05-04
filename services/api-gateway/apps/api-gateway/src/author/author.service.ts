import { Injectable, OnModuleInit, Inject } from '@nestjs/common';
import { ClientGrpc } from '@nestjs/microservices';
import { IAuthorService, IAuthor } from '@app/ts-interfaces';
import { Observable } from 'rxjs';

@Injectable()
export class AuthorService implements OnModuleInit {
  private authorService: IAuthorService;

  constructor(@Inject('DATABASE_PACKAGE') private readonly databaseClient: ClientGrpc) { }

  onModuleInit() {
    this.authorService = this.databaseClient.getService<IAuthorService>('AuthorService');
  }

  getAuthor(): Observable<IAuthor> {
    return this.authorService.findOneById({ id: 42 });
  }

  getAuthorById(id: number): Observable<IAuthor> {
    return this.authorService.findOneById({ id });
  }
}
