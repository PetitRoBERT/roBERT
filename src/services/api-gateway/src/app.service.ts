import { Injectable, OnModuleInit, Inject } from '@nestjs/common';
import { DatabaseService } from './interfaces/database';
import { ClientGrpc } from '@nestjs/microservices';
import { Observable } from 'rxjs';
import { Author } from './author/interfaces/author';

@Injectable()
export class AppService implements OnModuleInit {
  private databaseService: DatabaseService;

  constructor(@Inject('DATABASE_PACKAGE') private readonly client: ClientGrpc) {

  }

  onModuleInit() {
    this.databaseService = this.client.getService<DatabaseService>('DatabaseService');
  }

  getAuthor(): Observable<Author> {
    return this.databaseService.findOneAuthorById({ id: 42 });
  }
}
