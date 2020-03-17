import { Injectable, OnModuleInit, Inject } from '@nestjs/common';
import { database } from '../../../interfaces/database/api';
import { ClientGrpc } from '@nestjs/microservices';
import { Observable } from 'rxjs';

@Injectable()
export class AppService implements OnModuleInit {
  private authorService: database.AuthorService;

  constructor(@Inject('DATABASE_PACKAGE') private readonly client: ClientGrpc) {}

  onModuleInit() {
    this.authorService = this.client.getService<database.AuthorService>('AuthorService');
  }

  getAuthor(): Observable<database.Author> {
    return this.authorService.findOneById({ id: 42 });
  }
}
