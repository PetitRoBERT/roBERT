import { Controller, Logger } from '@nestjs/common';
import { AuthorService } from './author.service';
import { GrpcMethod } from '@nestjs/microservices';
import { IAuthorById, IAuthor } from '@app/ts-interfaces';

@Controller('author')
export class AuthorController {
  private logger = new Logger('AuthorController');

  constructor(private readonly authorService: AuthorService) {}

  // TODO: we should be able to remove annotation, if it's well named
  @GrpcMethod('AuthorService', 'FindOneById')
  findOneById(authorById: IAuthorById): IAuthor {
    this.logger.log(`Requesting author with id ${authorById.id.toString()}`);
    return this.authorService.findOneById(authorById.id);
  }
}
